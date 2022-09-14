import os
import sys


if __name__ == '__main__':
    # check if there is a path to the key in the arguments
    if len(sys.argv) > 1:
        key_path = sys.argv[1]
    else:
        key_path = input('Enter the path to the key: ')
    # join the paths
    private_path = os.path.join(key_path, 'private.pem')
    public_path = os.path.join(key_path, 'public.pem')

    # check if the key exists
    if os.path.isfile(private_path) or os.path.isfile(public_path):
        print('Error: there is already a key in this path')
        sys.exit(1)

    # generate both private and public keys
    os.system(f'openssl genpkey -algorithm X25519 -out {private_path}')
    os.system(f'openssl pkey -in {private_path} -pubout -out {public_path}')

    print('Done')