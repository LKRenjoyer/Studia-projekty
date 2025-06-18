'''
dziala w O(n) sprawdza dystans dla mozliwych poprawnych dla specyfikacji stringow 
przejscie z np 001111000... -> 000111100... w O(1), dokonujemy 2 sprawdzen
na koncach segmentu jedynek (technika sliding window)
'''


def opt_dist(txt, d) -> str :
    curr = ['1' if x < d else '0' for x in range(len(txt))]
    distance = len(list(filter(lambda x : x[0] != x[1], zip(curr,txt))))
    result = distance
    for step in range(d,len(txt)) :
        curr[step] = '1'
        curr[step-d] = '0'
        distance += -1 if txt[step] == '1' else 1
        distance += -1 if txt[step-d] == '0' else 1
        result = min(result,distance)
    return result


if __name__ in "__main__" :
    with open("zad4_input.txt",'r') as inp :
        with open("zad4_output.txt",'w') as out:
            for line in inp :
                txt, d = line.split(' ')
                out.write(str(opt_dist(str(txt),int(d)))+'\n')
    