import socket
import select

frontend_flow_id = b"\x00\x00\x00\x00"
ngap_setup_request = b"\x00\x15\x005\x00\x00\x04\x00\x1b\x00\x08\x00\x02\xf89\x03\x80\x00\x04\x00R@\x09\x03\x00Nervion\x00f\x00\x10\x00\x00\x00\x00\x01\x00\x02\xf89\x00\x00\x10\x08\x00\x00\x01\x00\x15@\x01@"


def main():
    target_host = "127.0.0.1"
    target_port = 9977

    # Send 10 bytes over UDP and print the response
    client = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    client.sendto(frontend_flow_id + ngap_setup_request, (target_host, target_port))
    while True:
        ready_to_read, _, _ = select.select([client], [], [], 1)
        if ready_to_read:
            data, addr = client.recvfrom(4096)
            print(data)
            break
        else:
            continue
    client.close()


if __name__ == "__main__":
    main()
