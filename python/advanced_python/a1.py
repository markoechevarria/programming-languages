import requests
import time

class ServiceUnavailableError(Exception):
    pass

def circuit_breaker(max_fallos: int, tiempo_espera: int):
    def decorator(func):

        fallos = 0
        ultimo_fallo_time = 0
        circuito_abierto = False

        def wrapper(*args, **kwargs):
            print(f"\nConsulting url: {args}")
            nonlocal fallos, ultimo_fallo_time, circuito_abierto

            if circuito_abierto:
                if time.time() - ultimo_fallo_time > tiempo_espera:
                    print("Retrying")
                    circuito_abierto = False
                else:
                    raise ServiceUnavailableError("Circuito abierto")

            try:
                result = func(*args, **kwargs)
                fallos = 0
                return result
            except Exception as e:
                fallos += 1
                ultimo_fallo_time = time.time()
                print(f"Detected error ({fallos}/{max_fallos})")

                if fallos >= max_fallos:
                    circuito_abierto = True
                    print("Circuito abierto")

                raise e
        return wrapper
    return decorator

@circuit_breaker(max_fallos=3, tiempo_espera=5)
def make_request(url: str):
    res = requests.get(url)
    return res

urls = [
    "https://www.google.com",
    "https://www.googleeee.com",
    "https://www.googleeee.com",
    "https://www.googleeee.com",
    "https://www.google.com",
    "https://www.googleeee.com",
    "https://www.googleeee.com",
    "https://www.googleeee.com",
]

for url in urls:
    try: 
        response = make_request(url)
        print(f"Exito: {url} - Status: {response.status_code}")
    except ServiceUnavailableError as e:
        print("Accion abortada")
    except Exception:
        print(f"Error controlado al consultar {url}")
