import time
import requests
import argparse
import keyboard
import sys

EXIT_FLAG = False

def exit_flag(_):
    global EXIT_FLAG
    EXIT_FLAG = True
    keyboard.unhook_all()

class Checkip: 

    def __init__(self, ip, port = "", route = ""):
        self.ip = ip
        self.port = port
        self.route = route

        if port == "" and route == "":
            self.url = f"http://{self.ip}"
        elif port == "" and route != "": 
            self.url = f"http://{self.ip}/{self.route}"
        elif port != "" and route == "": 
            self.url = f"http://{self.ip}:{self.port}"
        else: 
            self.url = f"http://{self.ip}:{self.port}/{self.route}"


    def check(self):

        global EXIT_FLAG

        keyboard.on_press_key("x", exit_flag)

        print(f"Checking {self.url}")
        print(f"Tap 'x' to exit")
        
        while not EXIT_FLAG:

            time.sleep(1)

            if EXIT_FLAG: break

            try:
                request = requests.get(self.url, timeout=3)
                if request.status_code == 200:
                    print("")
                    print("")
                    print("#################")
                    print("#################")
                    print("##             ##")
                    print("##  AVAILABLE  ##")
                    print("##             ##")
                    print("#################")
                    print("#################")
                    print("")
                    print("")
                    EXIT_FLAG = True

            except requests.exceptions.Timeout as e:
                sys.stdout.write('x')
                sys.stdout.flush()
                pass

            except requests.exceptions.RequestException as e:
                print(f"conection error")
                EXIT_FLAG = False

        print(f"Stop Cheking {self.url}")


def main():

    parser = argparse.ArgumentParser()
    parser.add_argument( "-i", "--ip", help="ip direction: 'example.com' or '10.10.10.10' ")
    parser.add_argument( "-p", "--port", help="port number: '1000' ")
    parser.add_argument( "-r", "--route", help="route direction: 'path/path2/path3' ")
    args = parser.parse_args()

    if not args.port and not args.route:
        checkip = Checkip(ip=args.ip)
    elif not args.port and args.route:
        checkip = Checkip(ip=args.ip, route=args.route) 
    elif not args.route and args.port: 
        checkip = Checkip(ip=args.ip, port=args.port)
    else:
        checkip = Checkip(ip=args.ip, route=args.route, port=args.port)

    checkip.check()

if __name__ == "__main__":
    import sys
    main()
