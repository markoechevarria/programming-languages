import time

def timer(func):
    def decorator(*args, **kwargs):
        initial_time = time.time()
        
        res = func(*args, **kwargs)

        final_time = time.time()

        print(f"Total time: {final_time - initial_time}")

        return res
    return decorator

@timer
def something(sleep: int = 3):
    time.sleep(sleep)
    print("Function completed")

for a in range(10):
    something(a)
