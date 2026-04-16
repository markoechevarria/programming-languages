import random
from pathlib import Path
from typing import TypedDict, Generator, Iterable, Dict

def create_csv( file: str, rows: int):
    estados = ['APROBADO', 'VENCIDO', 'PENDIENTE', 'SUBSANADO', 'EVALUADO', 'DESESTIMADO']
    with open(file, 'a') as f:
        for i in range(rows):
            line = f'{i},{int(random.random()*10000)},{random.choice(estados)}\n'
            f.write(line) 
    
def lector_datos(fuente: str) -> Generator[Dict]:
    if not Path(fuente).exists():
        raise FileNotFoundError(f"File not exists")

    with open(fuente, 'r') as f:
        for row in f:
            strip_line = row.strip().split(',')

            if len(strip_line) == 3:
                yield {
                    'id': int(strip_line[0]),
                    'monto': float(strip_line[1]),
                    'estado': str(strip_line[2])
                }

def filtros_pendientes(it: Iterable[Dict], estado: str):
    for dato in it:
        if dato['estado'] == estado:
            yield dato

def conversor_divisa( it: Iterable[Dict], cambio: int | float):
    for dato in it:
        dato['monto'] = round( dato['monto'] * cambio , 2)
        yield dato

def acumulador_lotes(it: Iterable[Dict], tamano_lote: int, destino: str):
    Path(destino).mkdir(exist_ok=True)
    lote= []
    contador_archivos = 0

    for row in it:
        lote.append(f"{row['id']}, {row['monto']}, {row['estado']}\n")

        if len(lote) >= tamano_lote:
            contador_archivos += 1
            path = Path(destino) / f'Lote_{contador_archivos}.csv'

            with open( path, 'w') as f: 
                f.writelines(lote)
            print(f"File {path} created.")
            lote = []

if __name__ == '__main__':
    archivo_input = 'data.csv'

    datos = lector_datos(archivo_input)
    datos_filtrados = filtros_pendientes(datos, "PENDIENTE")
    datos_convertidos = conversor_divisa(datos_filtrados, 0.92)

    acumulador_lotes(datos_convertidos, 100, "PENDIENTES")

