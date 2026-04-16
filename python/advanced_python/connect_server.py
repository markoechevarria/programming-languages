import requests

def retrying(intentos: int =3):
    def decorator(func):
        def wrapper(*args, **kwargs):
            
            for i in range(intentos):
                try:
                    result = func(*args, **kwargs)
                    print("Connection successful")
                    return result
                except Exception as e:
                    print(f"Error n. {i+1} ")
            print("Max num of errors reached. Exiting.")

        return wrapper
    return decorator 

@retrying(5)
def conectar_servidor(url):
    print(f"\nTrying to connect to {url}")
    return requests.get(url)

result1 = conectar_servidor("https://www.googleeee.com")
if result1: print(result1.text)

result2 = conectar_servidor("https://www.google.com")
print(result2.text[:100])
