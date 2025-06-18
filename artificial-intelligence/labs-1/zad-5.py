import random
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

NON_OPT = 5

class Nonogram :
    def __init__(self,height, width, rows, cols) : 
        self.height = height
        self.width = width
        self.rows_spec = rows
        self.cols_spec = cols
        self.image = self.scramble_image()
        #self.limit = 10000
    def row(self, ind) :
        return self.image[ind]
    def col(self,ind) :
        res = []
        for row in self.image :
            res.append(row[ind])
        return res
    def scramble_image(self) :
        return [[random.sample([0,1],1)[0] for _ in range(self.width)] for _ in range(self.height)]
    def opt_dist(self, txt : list[int], d : int) -> str :
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
        #print(self)
        #print('-'*80)
        while True :
            next_step = self.choose_next()
            if next_step == False :
                break
            else :
                opt = random.randint(0,100)
                if opt <= NON_OPT :
                    row_ind = random.choice(range(self.height))
                    col_ind = random.choice(range(self.width))
                else :
                    row_ind, col_ind = self.choose_minimal(next_step)
                #print(row_ind,' ',col_ind)
                self.image[row_ind][col_ind] ^= 1

    def __str__(self) :
        return '\n'.join([''.join(['.' if x == 0 else '#' for x in row]) for row in self.image])

def get_specs(inp = "zad5_input.txt") :
    rows = []
    cols = []
    with open(inp, 'r', encoding='utf-8') as file :
        x, y = [int(spec) for spec in file.readline().split(' ')]
        for _ in range(x) :
            rows.append(int(file.readline()))
        for _ in range(y) :
            cols.append(int(file.readline()))
    return x,y,rows,cols




if __name__ in "__main__" :
    nonogram = Nonogram(*get_specs())
    nonogram.solve()
    with open('zad5_output.txt','w') as out :
        out.write(str(nonogram))


