use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Debug)]
struct Fecha{
    dia: u8,
    mes: u8,
    año: u32
}
impl Fecha{
    fn new(dia: u8, mes: u8, año: u32) -> Fecha {
        Fecha { dia, mes, año }
    }

    fn es_fecha_valida(&self) -> bool {
        if self.mes <= 12 {
            match self.mes {
                1 => if self.dia <= 31 { return true },
                2 => if self.dia <= 28 { return true }else if self.es_bisiesto() && self.dia == 29 { return true },
                3 => if self.dia <= 31 { return true },
                4 => if self.dia <= 30 { return true },
                5 => if self.dia <= 31 { return true },
                6 => if self.dia <= 30 { return true },
                7 => if self.dia <= 31 { return true },
                8 => if self.dia <= 31 { return true },
                9 => if self.dia <= 30 { return true },
                10 => if self.dia <= 31 { return true },
                11 => if self.dia <= 30 { return true },
                12 => if self.dia <= 31 { return true },
                other => return false
            }
        }
        false
    }

    fn es_bisiesto(&self) -> bool {
        if self.año % 100 == 0 {
            return self.año % 400 == 0;
        }
        self.año % 4 == 0
    }

    fn sumar_dias(&mut self, dias: u8) {
        if self.es_fecha_valida(){
            let mut meses = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            let mut dia = dias + self.dia;
            while dia > meses[self.mes as usize -1] {
                if self.es_bisiesto(){ meses[1] = 29 }else{ meses [1] = 28}
                dia = dia - meses[self.mes as usize -1];
                if self.mes == 12{
                    self.mes = 1;
                    self.año = self.año + 1;
                }else{
                    self.mes = self.mes + 1;
                }
            }
            self.dia = dia;
        }else{
            println!("No es una fecha valida, no se suman los dias.");
        }
    }

    fn restar_dias(&mut self, dias: u8) {
        if self.es_fecha_valida(){
            let mut meses = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            let mut dia = dias + self.dia;
            while dia > meses[self.mes as usize -1] {
                if self.es_bisiesto(){ meses[1] = 29 }else{ meses [1] = 28}
                dia = dia - meses[self.mes as usize -1];
                if self.mes == 1{
                    self.mes = 12;
                    self.año = self.año - 1;
                }else{
                    self.mes = self.mes - 1;
                }
            }
            self.dia = dia;
        }else{
            println!("No es una fecha valida, no se suman los dias.");
        }
    }

    fn es_mayor(&self, una_fecha: Fecha) -> bool {
        if self.dia > una_fecha.dia || self.mes > una_fecha.mes || self.año > una_fecha.año {
            return true;
        }
        false
    }

    fn imprimir(&self) -> String {
        self.dia.to_string() + &String::from("/") + &self.mes.to_string() + &String::from("/") + &self.año.to_string()
    }
}

#[derive(Debug)]
enum Animal{
    Perro,
    Gato,
    Caballo,
    Otro
}
#[derive(Debug)]
struct Veterinaria{
    nombre: String,
    direccion: String,
    id: i32,
    atencion: VecDeque<Mascota>,
    finalizado: Vec<Finalizado>
}
#[derive(Debug)]
struct Mascota{
    nombre: String,
    edad: i32,
    animal: Animal,
    dueño: Dueño
}
impl Mascota{
    fn new(nombre: String, edad: i32, animal: Animal, dueño: Dueño) -> Mascota {
        Mascota { nombre, edad, animal, dueño }
    }
}
#[derive(Debug)]
struct Dueño{
    nombre: String,
    direccion: String,
    telefono: String
}
impl Dueño{
    fn new(nombre: String, direccion: String, telefono: String) -> Dueño{
        Dueño { nombre, direccion, telefono }
    }
}
#[derive(Debug)]
struct Finalizado{
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    visita: Option<Fecha>
}
impl Finalizado{
    fn new(mascota: Mascota, diagnostico: String, tratamiento: String, visita: Option<Fecha>) -> Finalizado {
        Finalizado { mascota, diagnostico, tratamiento, visita }
    }
}
impl Veterinaria{
    fn new(nombre: String, direccion: String, id: i32, atencion: VecDeque<Mascota>, finalizado: Vec<Finalizado>) -> Veterinaria {
        Veterinaria { nombre, direccion, id, atencion, finalizado }
    }

    fn agregar_mascota(&mut self, mascota: Mascota){
        self.atencion.push_back(mascota);
    }

    fn agregar_mascota_prioritaria(&mut self, mascota: Mascota) {
        self.atencion.push_front(mascota);
    }

    fn atender_mascota(&mut self, diagnostico: String, tratamiento: String, visita: Option<Fecha>) {
        let aux = self.atencion.pop_front();
        if let Some(data) = aux{
            let atendido = Finalizado::new(data, diagnostico, tratamiento, visita);
            self.finalizado.push(atendido);
        }
    }

    fn eliminar_mascota(&mut self, posicion: i32) {
        self.atencion.remove(posicion as usize);
    }

    fn buscar_mascota(&self, nombre: String, dueño: String, telefono: String) -> bool {
        for m in &self.atencion{
            if m.nombre == nombre && m.dueño.nombre == dueño && m.dueño.telefono == telefono {
                return true
            }
        }
        false
    }

    fn mod_diag(&mut self, diagnostico: String, posicion: i32) {
        self.finalizado[posicion as usize].diagnostico = diagnostico; 
    }

    fn mod_fecha(&mut self, fecha: Option<Fecha>, posicion: i32) {
        self.finalizado[posicion as usize].visita = fecha;
    }

    fn eliminar_atencion(&mut self, posicion: i32) {
        self.finalizado.remove(posicion as usize);
    }

    fn emails_a_enviar(&self, mut fecha_de_hoy: Fecha, dias: u8) -> Vec<String> {
        fecha_de_hoy.sumar_dias(dias);
        let mut emails = Vec::new();
        for i in 0..self.finalizado.len() {
            if let Some(data) = self.finalizado.get(i) {
                if data.visita == Some(fecha_de_hoy) {
                    if !emails.contains(&data.mascota.dueño.direccion) {
                        emails.push(data.mascota.dueño.direccion.clone());
                    }
                }
            }
        }
        emails
    }
}

pub fn exe() {
    let mut veterinaria = Veterinaria::new(String::from("Veterinaria de augus"), String::from("Calle 12 n° 6267"), 1, VecDeque::new(), Vec::new());

    veterinaria.agregar_mascota(Mascota::new(String::from("Felipe"), 4, Animal::Gato, Dueño::new(String::from("Camila"), String::from("cami@email.com"), String::from("221 123-4567"))));

    veterinaria.agregar_mascota_prioritaria(Mascota::new(String::from("Magno"), 4, Animal::Perro, Dueño::new(String::from("Camila"), String::from("cami@email.com"), String::from("221 123-4567"))));

    veterinaria.agregar_mascota(Mascota::new(String::from("Pichi"), 10, Animal::Perro, Dueño::new(String::from("Augusto"), String::from("augus@email.com"), String::from("221 123-4567"))));

    veterinaria.atender_mascota(String::from("Alergia"), String::from("Corticoides"), Some(Fecha::new(25, 5, 2023)));

    veterinaria.atender_mascota(String::from("Luxacion de codo"), String::from("Que deje quieta la pata"), Some(Fecha::new(26, 5, 2023)));

    //veterinaria.atender_mascota(String::from("Sarna"), String::from("Mucho cariño"), Some(Fecha::new(27, 5, 2023)));

    println!("{:#?}", veterinaria);

    println!("Los mails para el dia de mañana son: {:?}", veterinaria.emails_a_enviar(Fecha::new(25, 5, 2023), 1));
}
