import os
import shutil
from pathlib import Path
import requests
import pandas as pd

root = Path.cwd() / os.getenv("UBICACION")
url = os.getenv("URL")
df = pd.read_excel(os.getenv("EXCEL"))
respuestas = 0

def subirArchivo(ruta_subida, archivo_local):
    global respuestas
    params = { 
        "ruta": ruta_subida, 
        "reemplazarArchivo": "false" 
    }

    try:
        with open(archivo_local , "rb") as file:
            file_to_upload = [ ('archivos', (Path(archivo_local).name, file)) ]
    
            print(f"Enviando solicitud POST a: {url} con parámetros: {params} del archivo {file_to_upload[0]}")

            
            response = requests.post(
                url,
                params=params,
                files=file_to_upload
            )

            print("---")
            print(f"Código de estado HTTP: {response.status_code}")
            print("Respuesta del servidor:")

            if (response.status_code == 200): 
                respuestas = respuestas + 1
    
        try:
            print(response.json())
        except requests.exceptions.JSONDecodeError:
            print(response.text)

    except Exception as e:
        print(f"Error al subir archivo {archivo_local}: {e}")
    
    finally:
        if archivo_local in locals() and not archivo_local.closed:
            archivo_local.close()

def changeCaracteres(ruta_archivo):
    replacements = { 'á':'a', 'é':'e', 'í':'i', 'ó':'o', 'ú':'u', 'Á': 'A', 'É':'E', 'Í':'I', 'Ó':'O', 'Ú':'U', ' ':'_', '-':'_', '&':'', 'ñ':'n', 'Ñ':'N'  }

    ruta_archivo = Path(ruta_archivo)
    nombre_original = ruta_archivo.name
    nuevo_nombre = nombre_original.translate(str.maketrans(replacements))
    nuevo_nombre = nuevo_nombre.replace('__', '_')
    nuevo_nombre = nuevo_nombre.replace('___', '_')
    nuevo_nombre = nuevo_nombre.replace(',', '')
    ruta_reconstruida = ruta_archivo.parent / nuevo_nombre

    try:
        os.rename(ruta_archivo, ruta_reconstruida )
        print( "Ruta original:", ruta_archivo, "\nRuta nueva:", ruta_reconstruida, "\n\n")
    except Exception as e:
        print("No se pudo renombrar el archivo/directorio\nError:", e, "\n")


def recorrer(ruta):
    for elemento in list(ruta.iterdir()):
        if elemento.is_file():
            changeCaracteres(elemento)
        elif elemento.is_dir():
            if not any(elemento.iterdir()):
                changeCaracteres(elemento)
            else:
                recorrer(elemento)
    changeCaracteres(ruta)

def obtenerIdDepartamento(ruta):
    componentes = str(ruta).split(os.sep)
    unidad = list( filter(lambda componente: componente.startswith("MCC"), componentes))
    if unidad:
        unidad = unidad[0].replace("_", "-")
        try:
            return int( df[ df['UNIDAD'] == unidad]['INDICE'].values[0] )
        except:
            return -1

    else: return -1

def obtenerNombreDepartamento(ruta):
    componentes = str(ruta).split(os.sep)
    unidad = list( filter(lambda componente: componente.startswith("MCC"), componentes))
    return unidad

def obtenerListaDepartamentos(root):
    root = Path(root)
    lista = []

    departamentos = root / "2)_DEPARTAMENTOS"
    departamentos_lista = list(departamentos.iterdir())

    for departamento in departamentos_lista:
        lista.append(departamento.name)
    return lista

def generarSql(departamentoId, TipoArchivoId, Url, Texto):
    return f"(NEWID(), {departamentoId}, {TipoArchivoId}, '{Url}', '{Texto}'), "

def insertarCompartidos(root):
    root = Path(root)
    sql = []
    
    info_general = root / "3)_INFORMACION_GENERAL"
    mantenimiento_garantias = root / "5)_MANTENIMIENTO_Y_GARANTIAS"
    
    acabados_mobiliarios_equipos = root / "4)_ESPECIFICACIONES_TECNICAS" / "ACABADOS_MOBILIARIOS_EQUIPOS"
    
    ingreso_sala_comedor_pasadizo = acabados_mobiliarios_equipos / "1)_INGRESO_SALA_COMEDOR_PASADIZO"
    cocina = acabados_mobiliarios_equipos / "2)_COCINA"
    lavanderia = acabados_mobiliarios_equipos / "3)_LAVANDERIA"
    banio_principal = acabados_mobiliarios_equipos / "4)_BANIO_PRINCIPAL"
    banio_secundario = acabados_mobiliarios_equipos / "5)_BANIO_SECUNDARIO"
    dormitorios = acabados_mobiliarios_equipos / "6)_DORMITORIOS"
    balcon = acabados_mobiliarios_equipos / "8)_BALCON"
    
    info_general = list(info_general.iterdir())
    mantenimiento_garantias = list(mantenimiento_garantias.iterdir())
    
    ingreso_sala_comedor_pasadizo = list(ingreso_sala_comedor_pasadizo.iterdir())
    cocina = list(cocina.iterdir())
    lavanderia = list(lavanderia.iterdir())
    banio_principal = list(banio_principal.iterdir())
    banio_secundario = list(banio_secundario.iterdir())
    dormitorios = list(dormitorios.iterdir())
    balcon = list(balcon.iterdir())

    for a in info_general:
        subirArchivo( 'INMOBILIARIA_MCC/GLOBAL/INFORMACION_GENERAL' , a)
        sql.append(generarSql(0,11,f'INMOBILIARIA_MCC/GLOBAL/INFORMACION_GENERAL/{a.name}', f'{a.name}'))

    for a in mantenimiento_garantias:
        subirArchivo( 'INMOBILIARIA_MCC/GLOBAL/MANTENIMIENTO_DE_GARANTIAS' , a)
        sql.append(generarSql(0,12,f'INMOBILIARIA_MCC/GLOBAL/MANTENIMIENTO_DE_GARANTIAS/{a.name}', f'{a.name}'))

    for a in ingreso_sala_comedor_pasadizo:
        subirArchivo( 'INMOBILIARIA_MCC/GLOBAL/ACABADOS_MOBILIARIOS_EQUIPOS/INGRESO_SALA_COMEDOR_PASADIZO' , a)
        sql.append(generarSql(0,1,f'INMOBILIARIA_MCC/GLOBAL/ACABADOS_MOBILIARIOS_EQUIPOS/INGRESO_SALA_COMEDOR_PASADIZO/{a.name}', f'{a.name}'))

    for a in cocina:
        subirArchivo( 'INMOBILIARIA_MCC/GLOBAL/ACABADOS_MOBILIARIOS_EQUIPOS/COCINA' , a)
        sql.append(generarSql(0,2,f'INMOBILIARIA_MCC/GLOBAL/ACABADOS_MOBILIARIOS_EQUIPOS/COCINA/{a.name}', f'{a.name}'))

    for a in lavanderia:
        subirArchivo( 'INMOBILIARIA_MCC/GLOBAL/ACABADOS_MOBILIARIOS_EQUIPOS/LAVANDERIA' , a)
        sql.append(generarSql(0,3,f'INMOBILIARIA_MCC/GLOBAL/ACABADOS_MOBILIARIOS_EQUIPOS/LAVANDERIA/{a.name}', f'{a.name}'))

    for a in banio_principal:
        subirArchivo( 'INMOBILIARIA_MCC/GLOBAL/ACABADOS_MOBILIARIOS_EQUIPOS/BANIO_PRINCIPAL' , a)
        sql.append(generarSql(0,4,f'INMOBILIARIA_MCC/GLOBAL/ACABADOS_MOBILIARIOS_EQUIPOS/BANIO_PRINCIPAL/{a.name}', f'{a.name}'))

    for a in banio_secundario:
        subirArchivo( 'INMOBILIARIA_MCC/GLOBAL/ACABADOS_MOBILIARIOS_EQUIPOS/BANIO_SECUNDARIO' , a)
        sql.append(generarSql(0,5,f'INMOBILIARIA_MCC/GLOBAL/ACABADOS_MOBILIARIOS_EQUIPOS/BANIO_SECUNDARIO/{a.name}', f'{a.name}'))

    for a in dormitorios:
        subirArchivo( 'INMOBILIARIA_MCC/GLOBAL/ACABADOS_MOBILIARIOS_EQUIPOS/DORMITORIOS' , a)
        sql.append(generarSql(0,6,f'INMOBILIARIA_MCC/GLOBAL/ACABADOS_MOBILIARIOS_EQUIPOS/DORMITORIOS/{a.name}', f'{a.name}'))
        
    for a in balcon:
        subirArchivo( 'INMOBILIARIA_MCC/GLOBAL/ACABADOS_MOBILIARIOS_EQUIPOS/BALCON' , a)
        sql.append(generarSql(0,9,f'INMOBILIARIA_MCC/GLOBAL/ACABADOS_MOBILIARIOS_EQUIPOS/BALCON/{a.name}', f'{a.name}'))

    return sql

def insertarUnicos(root):
    root = Path(root)
    sql = []
    
    departamentos = root / "2)_DEPARTAMENTOS"
    departamentos_lista = list(departamentos.iterdir())
    
    for departamento in departamentos_lista:
        if (obtenerIdDepartamento(departamento)) != -1:
            for indice, carpeta_documentos in enumerate( list(departamento.iterdir()) ):
                if not (indice == 2 or indice == 4) and any(carpeta_documentos.iterdir()):
                        for documento in list(carpeta_documentos.iterdir()):
                            if 'FICHA_DEL_PROPIETARIO' in str(documento):
                                subirArchivo(f'INMOBILIARIA_MCC/DEPARTAMENTOS/{obtenerNombreDepartamento(documento)[0].replace("_", "-")}/FICHA_PROPIETARIO' , documento)
                                sql.append(generarSql(obtenerIdDepartamento(documento),14,f'INMOBILIARIA_MCC/DEPARTAMENTOS/{obtenerNombreDepartamento(documento)[0].replace("_", "-")}/FICHA_PROPIETARIO/{documento.name}', f'{documento.name}'))

                            if 'MECANICAS_Y_GAS' in str(documento):
                                subirArchivo(f'INMOBILIARIA_MCC/DEPARTAMENTOS/{obtenerNombreDepartamento(documento)[0].replace("_", "-")}/PLANOS_E_INSTALACIONES' , documento)
                                sql.append(generarSql(obtenerIdDepartamento(documento),17,f'INMOBILIARIA_MCC/DEPARTAMENTOS/{obtenerNombreDepartamento(documento)[0].replace("_", "-")}/PLANOS_E_INSTALACIONES/{documento.name}', f'{documento.name}'))

                            if 'COMUNICACIONES' in str(documento):
                                subirArchivo(f'INMOBILIARIA_MCC/DEPARTAMENTOS/{obtenerNombreDepartamento(documento)[0].replace("_", "-")}/PLANOS_E_INSTALACIONES' , documento)
                                sql.append(generarSql(obtenerIdDepartamento(documento),19,f'INMOBILIARIA_MCC/DEPARTAMENTOS/{obtenerNombreDepartamento(documento)[0].replace("_", "-")}/PLANOS_E_INSTALACIONES/{documento.name}', f'{documento.name}'))

                            if 'SEGURIDAD' in str(documento):
                                subirArchivo(f'INMOBILIARIA_MCC/DEPARTAMENTOS/{obtenerNombreDepartamento(documento)[0].replace("_", "-")}/PLANOS_E_INSTALACIONES' , documento)
                                sql.append(generarSql(obtenerIdDepartamento(documento),18,f'INMOBILIARIA_MCC/DEPARTAMENTOS/{obtenerNombreDepartamento(documento)[0].replace("_", "-")}/PLANOS_E_INSTALACIONES/{documento.name}', f'{documento.name}'))

                            if 'AGUA' in str(documento) or 'DESAGUE' in str(documento):
                                subirArchivo(f'INMOBILIARIA_MCC/DEPARTAMENTOS/{obtenerNombreDepartamento(documento)[0].replace("_", "-")}/PLANOS_E_INSTALACIONES' , documento)
                                sql.append(generarSql(obtenerIdDepartamento(documento),16,f'INMOBILIARIA_MCC/DEPARTAMENTOS/{obtenerNombreDepartamento(documento)[0].replace("_", "-")}/PLANOS_E_INSTALACIONES/{documento.name}', f'{documento.name}'))

                            if 'CORRIENTE' in str(documento):
                                subirArchivo(f'INMOBILIARIA_MCC/DEPARTAMENTOS/{obtenerNombreDepartamento(documento)[0].replace("_", "-")}/PLANOS_E_INSTALACIONES' , documento)
                                sql.append(generarSql(obtenerIdDepartamento(documento),15,f'INMOBILIARIA_MCC/DEPARTAMENTOS/{obtenerNombreDepartamento(documento)[0].replace("_", "-")}/PLANOS_E_INSTALACIONES/{documento.name}', f'{documento.name}'))
    return sql

def start():
    sql1 = insertarCompartidos(root)
    sql2 = insertarUnicos(root)

    print(f"Se han procesado con exito {respuestas} de {len(sql1 + sql2)} archivos")

    return sql1 + sql2

start()