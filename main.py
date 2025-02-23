import socket

def main():
    h = "127.0.0.1"  
    p = 4444       
    try:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((host, port))
            print(f"Connected to {host}:{port}")
            while True:
                cmd = input("C2> ")
                if cmd.lower() in ["exit", "quit"]:
                    break
                s.sendall(cmd.encode())
                response = s.recv(4096).decode()
                print(response)
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()
