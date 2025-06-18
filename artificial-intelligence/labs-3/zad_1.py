import random
import time
import sys
import itertools
from collections import deque
from copy import deepcopy




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

def dedup(lista) :
    lista = sorted(lista)
    result = [lista[0]]
    for ind in range(1,len(lista)) :
        if lista[ind]!=lista[ind-1]:
            result.append(lista[ind])
    return result


class Nonogram :
    @measure
    def __init__(self,height, width, rows, cols) : 
        self.domains = {}
        self.height = height
        self.width = width
        self.image = [[-1 for _ in range(self.width)] for _ in range(self.height)]
        self.row_domains = [dedup(self.generate_domain(width, spec)) for spec in rows]
        self.col_domains = [dedup(self.generate_domain(height, spec)) for spec in cols]
        self.rows_possibility_count = []
        self.cols_possibility_count = []
        self.deducing_queue = deque()
        for _ in range(height) :
            self.rows_possibility_count.append([[0,0] for _ in range(self.width)])
        for _ in range(width) :
            self.cols_possibility_count.append([[0,0] for _ in range(self.height)])
        for row_ind, row_domain in enumerate(self.row_domains) :
            for possible_row in row_domain :
                for cell_ind, val in enumerate(possible_row) :
                    self.rows_possibility_count[row_ind][cell_ind][val]+=1
        for col_ind, col_domain in enumerate(self.col_domains) :
            for possible_col in col_domain :
                for cell_ind, val in enumerate(possible_col) :
                    self.cols_possibility_count[col_ind][cell_ind][val]+=1
        
    def delete_from_domain(self, item, ind, row_col) :
        match row_col :
            case 'row' :
                for cell_ind, val in enumerate(item) :
                    self.rows_possibility_count[ind][cell_ind][val] -= 1
                    if self.rows_possibility_count[ind][cell_ind][val] == 0 :
                        self.deducing_queue.append((ind,cell_ind))
            case 'col' :
                for cell_ind, val in enumerate(item) :
                    self.cols_possibility_count[ind][cell_ind][val] -= 1 
                    if self.cols_possibility_count[ind][cell_ind][val] == 0 :
                        self.deducing_queue.append((cell_ind, ind))
    
    def add_to_domain(self, item, ind, row_col) :
        match row_col :
            case 'row' :
                self.row_domains[ind].append(item)
                for cell_ind,val in enumerate(item) :
                    self.rows_possibility_count[ind][cell_ind][val] += 1
            case 'col' :
                self.col_domains[ind].append(item)
                for cell_ind,val in enumerate(item) :
                    self.cols_possibility_count[ind][cell_ind][val] +=1

    def contradiction(self) :
        empty_row = any(map(lambda dom : dom==[], self.row_domains))
        empty_col = any(map(lambda dom: dom==[], self.col_domains))
        return empty_row or empty_col

    def clear_domain(self, banned_value, row_ind, col_ind) :
        rows_to_keep = []
        rows_to_delete = []
        for row in self.row_domains[row_ind] :
            if row[col_ind] == banned_value :
                rows_to_delete.append(row)
            else :
                rows_to_keep.append(row)
        if rows_to_delete :
            self.row_domains[row_ind] = rows_to_keep

        cols_to_keep = []
        cols_to_delete = []
        for col in self.col_domains[col_ind] :
            if col[row_ind] == banned_value :
                cols_to_delete.append(col)
            else :
                cols_to_keep.append(col)
        if cols_to_delete :
            self.col_domains[col_ind] = cols_to_keep

        [self.delete_from_domain(item, row_ind, 'row') for item in rows_to_delete]
        [self.delete_from_domain(item, col_ind, 'col') for item in cols_to_delete]
        return [(row_ind, rows_to_delete),(col_ind, cols_to_delete)]

    def deduce(self) :
        delta = ([],[])
        if len(self.deducing_queue) == 0 :
            for x,y in itertools.product(range(self.height),range(self.width)) :
                if self.rows_possibility_count[x][y][0] == 0 \
                or self.rows_possibility_count[x][y][1] == 0 \
                or self.cols_possibility_count[y][x][0] == 0 \
                or self.cols_possibility_count[y][x][1] == 0 :
                    self.deducing_queue.append((x,y)) 
        while len(self.deducing_queue) != 0 :
            if self.contradiction() :
                break
            drow, dcol = None,None
            x,y = self.deducing_queue.popleft()
            if self.image[x][y] == -1 :
                
                if self.rows_possibility_count[x][y][0] == 0 \
                or self.cols_possibility_count[y][x][0] == 0 :
                    self.image[x][y] = 1
                    drow,dcol = self.clear_domain(0, x, y)
                    delta[0].append(drow)
                    delta[1].append(dcol)   
                if self.rows_possibility_count[x][y][1] == 0 \
                or self.cols_possibility_count[y][x][1] == 0 :
                    self.image[x][y] = 0
                    drow,dcol = self.clear_domain(1,x,y)
                    delta[0].append(drow)
                    delta[1].append(dcol)
        self.deducing_queue.clear()
        return not self.contradiction(), delta
    

    def hard_switch(self, x, y, value) :
        self.image[x][y] = value
        return self.clear_domain(value^1, x, y)
    
    def row_switch(self, ind, row) :
        if any(map(lambda x: x==(0,1) or x==(1,0), zip(self.image[ind],row))) :
            return None
        delta = ([],[])
        for y,value in enumerate(row) :
            if self.image[ind][y] == -1 :
                new_delta = self.hard_switch(ind,y,value)
                delta[0].append(new_delta[0])
                delta[1].append(new_delta[1])
        return delta

    def solve(self) :
        curr_img = deepcopy(self.image)
        possible, delta = self.deduce()
        drow, dcol = delta
        #print("before deduce")
        #print([len(x) for x in self.row_domains])

        #print(self)
        if not possible :            
            for ind, deleted_rows in drow :
                for del_row in deleted_rows :
                    self.add_to_domain(del_row,ind,'row')
            for ind, deleted_cols in dcol :
                for del_col in deleted_cols :
                    self.add_to_domain(del_col,ind, 'col')
            self.image = deepcopy(curr_img)
            return False
        #print(self)
        #print('-'*80)
        '''
        row = None
        min_val = 1e9
        for ind, val in enumerate(self.row_domains) :
            if len(val) > 1 and len(val) < min_val :
                min_val = len(val)
                row = ind
        print("after deduce")
        print([len(x) for x in self.row_domains])
        if row is None :
            return True        
        #row, col = random.choice(choices)
        '''
        image_after_deducing = deepcopy(self.image)
        '''
        r_dom = deepcopy(self.row_domains[row])
        for new_row in r_dom :
            delta = self.row_switch(row, new_row)
            print("after setting")
            print(self.row_domains[row])
            print([len(x) for x in self.row_domains])
            if delta is None :
                    continue
            else :
                    if not self.contradiction() :
                        if self.solve() :
                            return True
                    drows, dcols = delta
                    for ind, deleted_rows in drows :
                        for del_row in deleted_rows :
                            self.add_to_domain(del_row,ind,'row')
                    for ind, deleted_cols in dcols :
                        for del_col in deleted_cols :
                            self.add_to_domain(del_col,ind,'col')
                    self.image = deepcopy(image_after_deducing)
        '''
        x,y = None,None
        for i,j in itertools.product(range(self.height),range(self.width)) :
            if self.image[i][j] == -1 :
                x,y = i,j
                break
        if x is None :
            return True
        for value in [0,1] :
            drows, dcols = self.hard_switch(x,y,value)
            if not self.contradiction() :
                if self.solve() :
                    return True
            ind,deleted_rows = drows
            for del_row in deleted_rows :
                self.add_to_domain(del_row, ind, 'row')
            ind, deleted_cols = dcols
            for del_cols in deleted_cols :
                self.add_to_domain(del_cols, ind, 'col')
            self.image[x][y] = -1
        
        for ind, deleted_rows in drow :
            for del_row in deleted_rows :
                self.add_to_domain(del_row,ind,'row')
        for ind, deleted_cols in dcol :
            for del_col in deleted_cols :
                self.add_to_domain(del_col,ind, 'col')
        self.image = deepcopy(curr_img)

        return False

        


    def domain(self, l, seg_len : int) -> list[list[int]] :
        result = []
        mid = [1 for _ in range(seg_len)]
        for split_point in range(l-seg_len+1) :
            lef = [0 for _ in range(split_point)]
            rig = [0 for _ in range(split_point+seg_len,l)]
            result.append(lef+mid+rig)
        return result

    def generate_domain(self, l, specs : list[int]) :
        if sum(specs) + len(specs) - 1 > l :
            return []
        key = (l,str(specs))
        if key in self.domains :
            return self.domains[key]
        if len(specs) == 0 :
            return [[0 for _ in range(l)]]
        if len(specs) == 1 :
            self.domains[key] = self.domain(l,specs[0])
            return self.domains[key]
        result = []
        for split_point in range(l) : 
            if split_point >= specs[0] :
                lef = self.generate_domain(split_point, [specs[0]])
            else :
                continue
            mid = [0]
            if l-1-split_point >= sum(specs[1:]) + len(specs) - 2 :
                rig = self.generate_domain(l-1-split_point, specs[1:])
            else :
                continue
            for a,b in itertools.product(lef,rig) :
                result.append(a+mid+b)
        self.domains[key] = result
        return result

    def __str__(self) :
        return '\n'.join([''.join(['.' if x == 0 else ('#' if x == 1 else '?') for x in row]) for row in self.image])

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
    print(nonogram)
    with open('zad_output.txt','w') as file :
        file.write(str(nonogram))


