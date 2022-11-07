def lmao2(a):
    return 10


def lmao(a: int):
    return a + lmao2(2)

print("PreJIT " + str(lmao(3)))

lmao.__jit__()
print("PostJIT " + str(lmao(3)))