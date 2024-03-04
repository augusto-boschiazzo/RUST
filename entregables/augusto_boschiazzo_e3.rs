use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
struct Fecha{
    dia: u32,
    mes: u32,
    año: u32
}
impl Fecha{
    fn new(dia: u32, mes: u32, año: u32) -> Fecha {
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

    fn sumar_dias(&mut self, dias: u32) {
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

    fn restar_dias(&mut self, dias: u32) {
        if self.es_fecha_valida(){
            let mut meses = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            let mut dia = self.dia as i32 - dias as i32;
            while dia < 0 {
                if self.es_bisiesto(){ meses[1] = 29 }else{ meses [1] = 28}
                dia = dia + meses[self.mes as usize -1];
                if self.mes == 1{
                    self.mes = 12;
                    self.año = self.año - 1;
                }else{
                    self.mes = self.mes - 1;
                }
            }
            self.dia = dia as u32;
        }else{
            println!("No es una fecha valida, no se restan los dias.");
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

#[test]
fn test_bisiesto(){
    let bisiesto = Fecha::new(2, 5, 2012);
    assert!(bisiesto.es_bisiesto())
}

#[test]
fn test_fecha_valida(){
    let invalida = Fecha::new(30, 2, 2001);
    let valida = Fecha::new(29, 2, 2000);
    assert!(!invalida.es_fecha_valida());
    assert!(valida.es_fecha_valida());
}

#[test]
fn test_mayor(){
    let fecha = Fecha::new(12, 3, 2002);
    let mayor = Fecha::new(26, 8, 2002);
    assert!(mayor.es_mayor(fecha));
    assert!(!fecha.es_mayor(mayor));
    assert!(!mayor.es_mayor(mayor));
}

#[test]
fn test_sumar_dias(){
    let mut fecha = Fecha::new(3, 9, 2011);
    let mayor = Fecha::new(11, 9, 2011);
    fecha.sumar_dias(9);
    assert!(!mayor.es_mayor(fecha));
}

#[test]
fn test_restar_dias(){
    let fecha = Fecha::new(12, 8, 2011);
    let mut mayor = Fecha::new(11, 9, 2011);
    mayor.restar_dias(30);
    assert!(!mayor.es_mayor(fecha));
}

#[derive(Serialize, Deserialize, Debug)]
enum Animal{
    Perro,
    Gato,
    Caballo,
    Otro
}
    
#[derive(Serialize, Deserialize, Debug)]
struct Veterinaria{
    nombre: String,
    direccion: String,
    id: i32,
    atencion: VecDeque<Mascota>,
    finalizado: Vec<Finalizado>
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

    fn imprimir(&self){
        println!("Mascota: {:?}, Diagnostico: {}, Tratamiento: {}, Visita: {}", self.mascota, self.diagnostico, self.tratamiento, match self.visita { Some(fecha) => fecha.imprimir(), None => "No definida".to_string() });
    }
}

trait ManejarArc{
    fn crear(veterinaria: &Veterinaria){
        let mut arc = match OpenOptions::new().write(true).create(true).truncate(true).open("src/veterinaria.json") {
            Err(e) => panic!("No se pudo abrir. Motivo: {}", e),
            Ok(arc) => {arc}
        };
        let buf = match serde_json::to_string_pretty(veterinaria) {
            Err(e) => panic!("No se pudo serializar. Motivo: {}", e),
            Ok(buf) => {buf}
        };
        arc.write_all(buf.as_bytes()).unwrap();
    }

    fn abrir() -> Veterinaria{
        let mut arc = match OpenOptions::new().read(true).open("src/veterinaria.json") {
            Err(e) => panic!("No se pudo abrir. Motivo: {}", e),
            Ok(arc) => {arc}
        };
        let mut buf = String::new();
        arc.read_to_string(&mut buf).unwrap();
        let veterinaria: Veterinaria = match serde_json::from_str(&buf) {
            Err(e) => panic!("No se pudo deserializar. Motivo: {}", e),
            Ok(veterinaria) => {veterinaria}
        };
        veterinaria
    }

    fn set_veterinaria(veterinaria: Veterinaria){
        let buf = match serde_json::to_string_pretty(&veterinaria) {
            Err(e) => panic!("No se pudo transformar a JSON. Motivo: {}", e),
            Ok(buf) => {buf}
        };
        let mut arc = match OpenOptions::new().write(true).truncate(true).open("src/veterinaria.json") {
            Err(e) => panic!("No se pudo abrir. Motivo: {}", e),
            Ok(arc) => {arc}
        };
        arc.write_all(buf.as_bytes()).unwrap();
    }

    fn get_finalizado() -> Vec<Finalizado>{
        let mut arc = match OpenOptions::new().read(true).open("src/veterinaria.json") {
            Err(e) => panic!("No se pudo abrir. Motivo: {}", e),
            Ok(arc) => {arc}
        };
        let mut buf = String::new();
        arc.read_to_string(&mut buf).unwrap();
        let veterinaria: Veterinaria = match serde_json::from_str(&buf) {
            Err(e) => panic!("No se pudo deserializar. Motivo: {}", e),
            Ok(veterinaria) => {veterinaria}
        };
        veterinaria.finalizado
    }

    fn set_finalizado(finalizado: Vec<Finalizado>){
        let mut arc = match OpenOptions::new().read(true).write(true).open("src/veterinaria.json") {
            Err(e) => panic!("No se pudo abrir. Motivo: {}", e),
            Ok(arc) => {arc}
        };
        let mut buf = String::new();
        arc.read_to_string(&mut buf).unwrap();
        let mut veterinaria: Veterinaria = match serde_json::from_str(&buf) {
            Err(e) => panic!("No se pudo deserializar. Motivo: {}", e),
            Ok(veterinaria) => {veterinaria}
        };
        veterinaria.finalizado = finalizado;
        let buf = match serde_json::to_string_pretty(&veterinaria) {
            Err(e) => panic!("No se pudo transformar a JSON. Motivo: {}", e),
            Ok(buf) => {buf}
        };
        let mut arc = match OpenOptions::new().write(true).truncate(true).open("src/veterinaria.json") {
            Err(e) => panic!("No se pudo abrir. Motivo: {}", e),
            Ok(arc) => {arc}
        };
        arc.write_all(buf.as_bytes()).unwrap();
    }

    fn get_atencion() -> VecDeque<Mascota>{
        let mut arc = match OpenOptions::new().read(true).open("src/veterinaria.json") {
            Err(e) => panic!("No se pudo abrir. Motivo: {}", e),
            Ok(arc) => {arc}
        };
        let mut buf = String::new();
        arc.read_to_string(&mut buf).unwrap();
        let veterinaria: Veterinaria = match serde_json::from_str(&buf) {
            Err(e) => panic!("No se pudo deserializar. Motivo: {}", e),
            Ok(veterinaria) => {veterinaria}
        };
        veterinaria.atencion
    }

    fn set_atencion(atencion: VecDeque<Mascota>){
        let mut arc = match OpenOptions::new().read(true).write(true).open("src/veterinaria.json") {
            Err(e) => panic!("No se pudo abrir. Motivo: {}", e),
            Ok(arc) => {arc}
        };
        let mut buf = String::new();
        arc.read_to_string(&mut buf).unwrap();
        let mut veterinaria: Veterinaria = match serde_json::from_str(&buf) {
            Err(e) => panic!("No se pudo deserializar. Motivo: {}", e),
            Ok(veterinaria) => {veterinaria}
        };
        veterinaria.atencion = atencion;
        let buf = match serde_json::to_string_pretty(&veterinaria) {
            Err(e) => panic!("No se pudo transformar a JSON. Motivo: {}", e),
            Ok(buf) => {buf}
        };
        let mut arc = match OpenOptions::new().write(true).truncate(true).open("src/veterinaria.json") {
            Err(e) => panic!("No se pudo abrir. Motivo: {}", e),
            Ok(arc) => {arc}
        };
        arc.write_all(buf.as_bytes()).unwrap();
    }
}

impl ManejarArc for Veterinaria{}

impl Veterinaria{
    fn new(nombre: String, direccion: String, id: i32) -> Veterinaria {
        let finalizado: Vec<Finalizado> = Vec::new();
        let atencion: VecDeque<Mascota> = VecDeque::new();
        let veterinaria = Veterinaria { nombre, direccion, id, atencion, finalizado };
        <Veterinaria as self::ManejarArc>::crear(&veterinaria);
        veterinaria
    }

    fn agregar_mascota(&mut self, mascota: Mascota){
        let mut atencion = <Veterinaria as self::ManejarArc>::get_atencion();
        atencion.push_back(mascota);
        <Veterinaria as self::ManejarArc>::set_atencion(atencion);
    }

    fn agregar_mascota_prioritaria(&mut self, mascota: Mascota) {
        let mut atencion = <Veterinaria as self::ManejarArc>::get_atencion();
        atencion.push_front(mascota);
        <Veterinaria as self::ManejarArc>::set_atencion(atencion);
    }

    fn atender_mascota(&mut self, diagnostico: String, tratamiento: String, visita: Option<Fecha>) {
        let mut veterinaria = <Veterinaria as self::ManejarArc>::abrir();
        let aux = veterinaria.atencion.pop_front();
        if let Some(data) = aux{
            let atendido = Finalizado::new(data, diagnostico, tratamiento, visita);
            veterinaria.finalizado.push(atendido);
        }
        <Veterinaria as self::ManejarArc>::set_veterinaria(veterinaria);
    }

    fn eliminar_mascota(&mut self, posicion: i32) {
        let mut atencion = <Veterinaria as self::ManejarArc>::get_atencion();
        if atencion.len() < posicion as usize{
            println!("Posicion invalida.");
        }else{
            atencion.remove(posicion as usize);
        }
        <Veterinaria as self::ManejarArc>::set_atencion(atencion);
    }

    fn buscar_mascota(&self, nombre: String, dueño: String, telefono: String) -> bool {
        let atencion = <Veterinaria as self::ManejarArc>::get_atencion();
        for m in &atencion{
            if m.nombre == nombre && m.dueño.nombre == dueño && m.dueño.telefono == telefono {
                return true
            }
        }
        false
    }

    fn mod_diag(&mut self, diagnostico: String, posicion: i32) {
        let mut finalizado = <Veterinaria as self::ManejarArc>::get_finalizado();
        if finalizado.len() < posicion as usize{
            println!("Posicion invalida.");
        }else{
            finalizado[posicion as usize].diagnostico = diagnostico; 
        }
        <Veterinaria as self::ManejarArc>::set_finalizado(finalizado);
    }

    fn mod_fecha(&mut self, fecha: Option<Fecha>, posicion: i32) {
        let mut finalizado = <Veterinaria as self::ManejarArc>::get_finalizado();
        if finalizado.len() < posicion as usize{
            println!("Posicion invalida.");
        }else{
            finalizado[posicion as usize].visita = fecha;
        }
        <Veterinaria as self::ManejarArc>::set_finalizado(finalizado);
    }

    fn eliminar_atencion(&mut self, posicion: i32) {
        let mut finalizado = <Veterinaria as self::ManejarArc>::get_finalizado();
        if finalizado.len() < posicion as usize{
            println!("Posicion invalida.");
        }else{
            finalizado.remove(posicion as usize);
        }
        <Veterinaria as self::ManejarArc>::set_finalizado(finalizado);
    }
}

#[test]
fn crear_veterinaria(){
    Veterinaria::new("nombre".to_string(), "direccion".to_string(), 1);
}

#[test]
fn test_agregar_mascota(){
    let mut vet = Veterinaria::abrir();
    let dueño = Dueño::new("dueño".to_string(), "direccion dueño".to_string(), "telefono".to_string());
    let mascota = Mascota::new("mascota".to_string(), 4, Animal::Gato, dueño);
    vet.agregar_mascota(mascota);
}

#[test]
fn test_atender_mascota(){
    let mut vet = Veterinaria::abrir();
    vet.atender_mascota("diagnostico".to_string(), "tratamiento".to_string(), None);
}

#[test]
fn test_agregar_mascota_prioritaria(){
    let mut vet = Veterinaria::abrir();
    let dueño = Dueño::new("dueño".to_string(), "direccion dueño".to_string(), "telefono".to_string());
    let mascota = Mascota::new("mascota prio".to_string(), 1, Animal::Perro, dueño);
    vet.agregar_mascota_prioritaria(mascota);
}

#[test]
fn test_eliminar_mascota(){
    let mut vet = Veterinaria::abrir();
    vet.eliminar_mascota(0);
}

#[test]
fn test_buscar_mascota(){
    let vet = Veterinaria::abrir();
    assert!(vet.buscar_mascota("mascota".to_string(), "dueño".to_string(), "telefono".to_string()));
}

#[test]
fn test_mod_fecha(){
    let mut vet = Veterinaria::abrir();
    vet.mod_fecha(Some(Fecha::new(15, 6, 2023)), 0);
}

#[test]
fn test_mod_diag(){
    let mut vet = Veterinaria::abrir();
    vet.mod_diag("nuevo diagnostico".to_string(), 0);
}