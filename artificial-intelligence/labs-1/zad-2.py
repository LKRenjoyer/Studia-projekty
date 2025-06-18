from sortedcontainers import SortedSet
'''
init - przekazanie slow z pliku do obiektu SortedSet ( znany npp z cpp set naklepany w C )
process - funkcja dostajaca linijke, zwraca pare (optymalny tekst, wynik tekstu) (para dla rekurencji)
idea funkcji polega na podzieleniu tekstu na 2 czesci gdzie 1 czesc MUSI byc poprawnym slowem
(bo ją zostawimy bez spacji) a druga bedzie zalezec od wyniku na niej funkcji process,
gdy nie uda sie prawidlowo podzielic otrzymamy mocno minusowy wynik, co gwarantuje nam
przez to ze poprawna odpowiedz istnieje - wynik poprawny (gdyby bowiem jeden z fragmentow 
sie nie zgadzal to dla tego przedzialu dostaniemy ten ujemny wynik)
'''



def init(filename = "polish_words.txt") :
    global words
    
    with open(filename, 'r', encoding='utf-8') as file :
        words = SortedSet([line.strip() for line in file])

def process(txt : str) -> tuple[str,int] :
    global maxes
    if txt in words : 
        return (txt,len(txt)*len(txt))
    if txt in maxes :
        return maxes[txt]
    opt_val = -1000000000
    opt_text = ""
    for i in range(1,len(txt)) :
        curr_val = 0
        curr_text = ""
        if txt[:i] in words :
            if not txt[i:] in maxes :
                maxes[txt[i:]] = process(txt[i:])
            curr_text, curr_val = maxes[txt[i:]]
            curr_text = txt[:i]+' '+curr_text
            curr_val += i*i
            if curr_val > opt_val :
                opt_text = curr_text
                opt_val = curr_val
    return (opt_text,opt_val)
    

if __name__  in "__main__" :
    init()
    with open("zad2_input.txt",'r',encoding='utf-8') as file :
        with open("zad2_output.txt", 'w', encoding='utf-8') as out :
            for line in file :
                maxes = {}
                txt,val = process(line)
                out.write(txt+'\n')
            