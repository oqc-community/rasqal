from os.path import abspath, join, dirname

def get_qir_path(file_path):
    return abspath(join(dirname(__file__), "files", "qir", file_path))

def get_qir(file_path):
    with open(get_qir_path(file_path)) as ifile:
        return ifile.read()
