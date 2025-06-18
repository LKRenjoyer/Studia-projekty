import random
import time
import sys
'''
Nonogram : 
    inicjuje obrazek dla danego rozmiaru w sposob losowy, za pomoca scramble_image
    row, col - zwraca odpowiedni wiersz,kolumne w zaleznosci od podanego indexu
    opt_dist - patrz zad4
    choose_next - losuje kolumne lub wiersz, ktore nie spelniaja specyfikacji
    choose_minimal - zwraca koordynaty komorki dla ktorej zamiana wartosci w wybranym
    wczesniej wierszu lub kolumnie da najlepszy wynik 
    solve - stara sie rozwiazac obrazek, gdy nie udaje sie po (self.limit (1e4))
    iteracjach miesza obrazek i probuje ponownie (bez tego test3 czesto nie dzialal)

'''



def measure(fun) :
    def wrapper(*args, **kwargs) :
        st = time.time()
        v = fun(*args, **kwargs)
        print(f"Executed in {time.time() - st}")
        return v
    return wrapper


INF = 1000000000
NON_OPT = 0.2

memo = {}

class Nonogram :
    def __init__(self,height, width, rows, cols) : 
        self.height = height
        self.width = width
        self.rows_spec = rows
        self.cols_spec = cols
        self.image = self.scramble_image()

    def row(self, ind) :
        return self.image[ind]

    def col(self,ind) :
        res = []
        for row in self.image :
            res.append(row[ind])
        return res

    def scramble_image(self) :
        return [[random.sample([0,1],1)[0] for _ in range(self.width)] for _ in range(self.height)]

    def dist_for_one_segment(self, txt : list[int], d : int) -> int :
            if len(txt) < d :
                return INF
            curr = [1 if x < d else 0 for x in range(len(txt))]
            distance = len(list(filter(lambda x : x[0] != x[1], zip(curr,txt))))
            result = distance
            for step in range(d,len(txt)) :
                curr[step] = 1
                curr[step-d] = 0
                distance += -1 if txt[step] == 1 else 1
                distance += -1 if txt[step-d] == 0 else 1
                result = min(result,distance)
            return result
    def opt_dist(self, txt : list[int], d : list[int], suma = None) -> int :
        key = (''.join([str(a) for a in txt]),'|'.join([str(a) for a in d]))

        if suma is None :
            suma = sum(d) + len(d) - 1 - d[0]

        if key in memo :
            return memo[key]
        
        if len(d) == 1 :
            return self.dist_for_one_segment(txt, d[0])
        else :
            min_res = INF # inf
            for start_pos in range(d[0], len(txt)-suma+1) :
                res = 1 if txt[start_pos] == 1 else 0
                res += self.dist_for_one_segment(txt[:start_pos], d[0])
                res += self.opt_dist(txt[start_pos+1:],d[1:], suma-d[1]-1)
                min_res = min(min_res,res)
        memo[key] = min_res
        if min_res == INF :
            pass
            #print(txt,' ',d,' ',min_res)
        return min_res

    def choose_next(self) :
        res = []
        for i in range(self.height) :
            if self.opt_dist(self.row(i),self.rows_spec[i]) > 0 :
                res.append((i,-1))
        
        for i in range(self.width) :
            if self.opt_dist(self.col(i),self.cols_spec[i]) > 0 :
                res.append((-1,i))
        if res != [] :
            return random.sample(res,1)[0]
        else :
            return False

    def choose_minimal(self, choice) :
        def calc_change(row_ind, col_ind) -> int :
            row = self.row(row_ind)
            col = self.col(col_ind)
            row[col_ind] ^= 1
            col[row_ind] ^= 1
            res = self.opt_dist(self.row(row_ind), self.rows_spec[row_ind])
            res += self.opt_dist(self.col(col_ind), self.cols_spec[col_ind])
            row[col_ind] ^= 1
            col[row_ind] ^= 1
            return res
        min_sum = self.width + self.height
        if choice[1] == -1 :
            row_ind = choice[0]
            col_ind = -1
            for curr_col_ind in range(self.width) :
                curr_sum = calc_change(row_ind, curr_col_ind)
                if curr_sum < min_sum :
                    min_sum = curr_sum
                    col_ind = curr_col_ind
                    
        else :
            col_ind = choice[1]
            row_ind = -1
            for curr_row_ind in range(self.height) :
                curr_sum = calc_change(curr_row_ind, col_ind)
                if curr_sum < min_sum :
                    min_sum = curr_sum
                    row_ind = curr_row_ind
        return (row_ind, col_ind)
    def solve(self) :
        while True :
                
            next_step = self.choose_next()
            #print(self)
            #print('-'*80)
            if next_step == False :
                break
            else :
                opt = random.random()
                if opt <= NON_OPT :
                    row_ind = random.choice(range(self.height))
                    col_ind = random.choice(range(self.width))
                else :
                    row_ind, col_ind = self.choose_minimal(next_step)
                #print(row_ind,' ',col_ind)
                self.image[row_ind][col_ind] ^= 1
    def __str__(self) :
        return '\n'.join([''.join(['.' if x == 0 else '#' for x in row]) for row in self.image])

def get_specs():
    with open("zad_input.txt", 'r', encoding='utf-8') as file:
        content = file.read()
    lines = content.strip().split("\n")
    x, y = map(int, lines[0].split())
    rows = []
    cols = []
    for i in range(1, x+1):
        rows.append([int(a) for a in lines[i].split()])
    for i in range(1 + x, 1 + x + y):
        cols.append([int(a) for a in lines[i].split()])
    return x, y, rows, cols




if __name__ in "__main__" :
    nonogram = Nonogram(*get_specs())
    nonogram.solve()
    with open("zad_output.txt",'w') as file :
        file.write(str(nonogram))


