def tup(name, len):
    ret = '('
    for i in range(len):
        ret += f'{name}{i}, '
    return ret + ')'


for i in range(10):
    print(
        f'impl_unzip!({tup("T",i)}, {tup("A",i)}, {tup("s",i)}, {tup("t",i)});')
