from itertools import product


def B(i,j):
    return f"B_{i}_{j}"
def domains(variables) :
    return [f"[{', '.join(variables)}] ins 0..1"]
def assignments(triples) :
    return [', '.join([f"{B(i,j)} #= {value}" for i,j,value in triples])]
def one_sum(ind, sum_cs, row_col) :
    match row_col :
        case 'row' :
                lef = ','.join([B(ind,i) for i in range(width)])
        case 'col' :
                lef = ','.join([B(j,ind) for j in range(height)])
    return f"sum([{lef}], #=, {sum_cs})"
def all_sums(rows, cols) :
    row_sum = [one_sum(ind,sum_cs,'row') for ind,sum_cs in enumerate(rows)]
    col_sum = [one_sum(ind,sum_cs,'col') for ind,sum_cs in enumerate(cols)]
    return row_sum + col_sum
# A+D = 2 <=> B+C = 2
def one_square(i,j) :
    A = B(i,j)
    Bx = B(i,j+1)
    Cx = B(i+1,j)
    D = B(i+1,j+1)
    return f"{A}+{D} #= 2 #<==> {Bx} + {Cx} #= 2"
def all_squares() :
    return [one_square(i,j) for i,j in product(range(width-1),range(height-1))]
# B = 1 => A+C >= 1
def one_line(i,j,direction) :
    match direction :
        case 'down' :
            A = B(i,j)
            Bx = B(i,j+1)
            C = B(i,j+2)
        case 'right' :
            A = B(i,j)
            Bx = B(i+1,j)
            C = B(i+2,j)
    return f"{Bx} #= 1 #==> {A}+{C} #>= 1"
def all_lines() :
    d = [one_line(i,j,'down') for i,j in product(range(width),range(height-2))]
    r = [one_line(i,j,'right') for i,j in product(range(width-2),range(height))]
    return d + r

def generate_program(rows, cols, triples):
    global height, width
    
    height = len(rows)
    width = len(cols)
    vs = [B(i,j) for i,j in product(range(height),range(width))]
    constraints = domains(vs) \
                + all_sums(rows,cols) \
                + all_squares() \
                + all_lines() \
                + assignments(triples)
    writeln(':- use_module(library(clpfd)).')
    writeln('solve([' + ', '.join(vs) + ']) :- ')
    [writeln(c+',') for c in constraints]
    writeln('labeling([ff], [' +  ', '.join(vs) + ']).' )
    writeln(":- solve(X), write(X), nl.")


def writeln(s):
    output.write(s + '\n')

txt = open('zad_input.txt').readlines()
output = open('zad_output.txt', 'w')

rows = list(map(int, txt[0].split()))
cols = list(map(int, txt[1].split()))
triples = []

for i in range(2, len(txt)):
    if txt[i].strip():
        triples.append(map(int, txt[i].split()))

generate_program(rows, cols, triples)            
        

