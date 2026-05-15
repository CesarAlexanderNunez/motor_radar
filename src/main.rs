#[derive(Debug, Clone)]
struct Vuelo {
    id: String,
    altitud: u32, // Este será nuestra clave (key)
}

// Usamos Box<Nodo> porque el tamaño de un Nodo que contiene otros Nodos es infinito en tiempo de compilación (recursividad).
// Box coloca el nodo en el Heap y nos da un puntero de tamaño fijo en el Stack.

struct Nodo {
    vuelo: Vuelo,
    izquierdo: Option<Box<Nodo>>,
    derecho: Option<Box<Nodo>>,
    altura: i32,
}

impl Nodo {
    fn nuevo(vuelo: Vuelo) -> Self {
        Nodo {
            vuelo,
            izquierdo: None,
            derecho: None,
            altura: 1,
        }
    }
}

// --- UTILIDADES DE BALANCEO (NO MODIFICAR) ---

fn obtener_altura(nodo: &Option<Box<Nodo>>) -> i32 {
    nodo.as_ref().map_or(0, |n| n.altura)
}

fn actualizar_altura(nodo: &mut Nodo) {
    nodo.altura = 1 + std::cmp::max(
        obtener_altura(&nodo.izquierdo),
        obtener_altura(&nodo.derecho),
    );
}

fn obtener_balance(nodo: &Nodo) -> i32 {
    obtener_altura(&nodo.izquierdo) - obtener_altura(&nodo.derecho)
}

// Se utiliza Option::take() para mover la propiedad (ownership) del nodo hijo fuera del Option, dejando un None en su lugar. 
//En Rust, esto es necesario porque no podemos dejar un campo de una estructura vacío.

fn rotar_derecha(mut y: Box<Nodo>) -> Box<Nodo> {
    let mut x = y.izquierdo.take().expect("Error de radar");
    y.izquierdo = x.derecho.take();
    actualizar_altura(&mut y);
    x.derecho = Some(y);
    actualizar_altura(&mut x);
    x
}

fn rotar_izquierda(mut x: Box<Nodo>) -> Box<Nodo> {
    let mut y = x.derecho.take().expect("Error de radar");
    x.derecho = y.izquierdo.take();
    actualizar_altura(&mut x);
    y.izquierdo = Some(x);
    actualizar_altura(&mut y);
    y
}

// --- FUNCIÓN DE INSERCIÓN ---

fn insertar(nodo_opt: Option<Box<Nodo>>, vuelo: Vuelo) -> Box<Nodo> {
    let mut nodo = match nodo_opt {
        None => return Box::new(Nodo::nuevo(vuelo)),
        Some(n) => n,
    };

    if vuelo.altitud < nodo.vuelo.altitud {
        nodo.izquierdo = Some(insertar(nodo.izquierdo.take(), vuelo.clone()));
    } else if vuelo.altitud > nodo.vuelo.altitud {
        nodo.derecho = Some(insertar(nodo.derecho.take(), vuelo.clone()));
    } else {
        return nodo; 
    }

    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    // Caso Izquierda-Izquierda
    if balance > 1 && vuelo.altitud < nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        return rotar_derecha(nodo);
    }
    // Caso Derecha-Derecha
    if balance < -1 && vuelo.altitud > nodo.derecho.as_ref().unwrap().vuelo.altitud {
        return rotar_izquierda(nodo);
    }
    // Caso Izquierda-Derecha
    if balance > 1 && vuelo.altitud > nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        let hijo_izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
        return rotar_derecha(nodo);
    }
    // Caso Derecha-Izquierda
    if balance < -1 && vuelo.altitud < nodo.derecho.as_ref().unwrap().vuelo.altitud {
        let hijo_der = nodo.derecho.take().unwrap();
        nodo.derecho = Some(rotar_derecha(hijo_der));
        return rotar_izquierda(nodo);
    }
    
    nodo
}

fn buscar_vuelo(nodo: &Option<Box<Nodo>>, altitud: u32) -> Option<&Vuelo> {
    match nodo {
        None => None,
        Some(n) => {
            if altitud == n.vuelo.altitud {
                Some(&n.vuelo)
            } else if altitud < n.vuelo.altitud {
                buscar_vuelo(&n.izquierdo, altitud)
            } else {
                buscar_vuelo(&n.derecho, altitud)
            }
        }
    }
}

fn eliminar_vuelo(nodo_opt: Option<Box<Nodo>>, altitud: u32) -> Option<Box<Nodo>> {
    let mut nodo = match nodo_opt {
        None => return None,
        Some(n) => n,
    };

    if altitud < nodo.vuelo.altitud {
        nodo.izquierdo = eliminar_vuelo(nodo.izquierdo.take(), altitud);
    } else if altitud > nodo.vuelo.altitud {
        nodo.derecho = eliminar_vuelo(nodo.derecho.take(), altitud);
    } else {
        // Encontramos el vuelo a eliminar (Aterrizaje)
        if nodo.izquierdo.is_none() {
            return nodo.derecho.take(); // Caso 1 y 2: Sin hijos o solo hijo derecho
        } else if nodo.derecho.is_none() {
            return nodo.izquierdo.take(); // Caso 2: Solo hijo izquierdo
        }

        // Caso 3: Tiene dos hijos. Buscamos el sucesor in-orden (menor del lado derecho)
        let mut temp = nodo.derecho.as_ref().unwrap();
        while temp.izquierdo.is_some() {
            temp = temp.izquierdo.as_ref().unwrap();
        }
        let vuelo_sucesor = temp.vuelo.clone();
        
        // Reemplazamos los datos con los del sucesor
        nodo.vuelo = vuelo_sucesor.clone();
        // Eliminamos el nodo duplicado del subárbol derecho
        nodo.derecho = eliminar_vuelo(nodo.derecho.take(), vuelo_sucesor.altitud);
    }

    // Actualizamos la altura del nodo actual
    actualizar_altura(&mut nodo);

    // Revisamos si el árbol se desbalanceó
    let balance = obtener_balance(&nodo);

    // Caso Izquierda-Izquierda
    if balance > 1 && obtener_balance(nodo.izquierdo.as_ref().unwrap()) >= 0 {
        return Some(rotar_derecha(nodo));
    }
    // Caso Izquierda-Derecha
    if balance > 1 && obtener_balance(nodo.izquierdo.as_ref().unwrap()) < 0 {
        let hijo_izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
        return Some(rotar_derecha(nodo));
    }
    // Caso Derecha-Derecha
    if balance < -1 && obtener_balance(nodo.derecho.as_ref().unwrap()) <= 0 {
        return Some(rotar_izquierda(nodo));
    }
    // Caso Derecha-Izquierda
    if balance < -1 && obtener_balance(nodo.derecho.as_ref().unwrap()) > 0 {
        let hijo_der = nodo.derecho.take().unwrap();
        nodo.derecho = Some(rotar_derecha(hijo_der));
        return Some(rotar_izquierda(nodo));
    }

    Some(nodo)
}

fn main() {
    let mut radar: Option<Box<Nodo>> = None;
    
    // Simulación de entrada de vuelos
    let datos = vec![
        ("AV123", 5000), ("UA456", 3000), ("IB101", 2000),
        ("AF999", 4000), ("TA222", 3500), ("AM777", 6000),
    ];

    for (id, alt) in datos {
        let v = Vuelo { id: id.to_string(), altitud: alt };
        radar = Some(insertar(radar.take(), v));
    }

    println!("--- Radar de Control Aéreo (AVL) ---");
    // Aquí el estudiante debe invocar sus funciones de búsqueda y eliminación
    if let Some(vuelo) = buscar_vuelo(&radar, 3500) {
        println!("Vuelo encontrado: {}", vuelo.id);
    } else {
        println!("Vuelo no encontrado");
    }

    // --- Prueba de la Fase 3: Aterrizaje de Vuelos ---
    println!("\nIniciando maniobra de aterrizaje para el vuelo a 3000 pies...");
    radar = eliminar_vuelo(radar.take(), 3000);
    
    if let Some(vuelo) = buscar_vuelo(&radar, 3000) {
        println!("Error: El vuelo {} sigue en el radar.", vuelo.id);
    } else {
        println!("Aterrizaje exitoso. Vuelo a 3000 pies eliminado del radar.");
    }
}

