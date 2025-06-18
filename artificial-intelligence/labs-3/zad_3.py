import sys
from itertools import product

def V(i,j):
    return 'V%d_%d' % (i,j)
    
def domains(Vs):
    return [ q + ' in 1..9' for q in Vs ]
    
def all_different(Qs):
    return 'all_distinct([' + ', '.join(Qs) + '])'
    
def get_column(j):
    return [V(i,j) for i in range(9)] 
            
def get_row(i):
    return [V(i,j) for j in range(9)] 

def get_square(i,j):
    return [V(i+dx,j+dy) for dx,dy in product(range(3),range(3))]

def horizontal():   
    return [all_different(get_row(i)) for i in range(9)]

def vertical():
    return [all_different(get_column(j)) for j in range(9)]

def squares():
    return [all_different(get_square(3*i,3*j)) for i,j in product(range(3),range(3))]


def print_constraints(Cs, indent, d, file):
    position = indent
    file.write(indent * ' ')
    for c in Cs:
        file.write(c + ', ')
        position += len(c)
        if position > d:
            position = indent
            file.write('\n')
            file.write(indent * ' ')

      
def sudoku(assigments):
    variables = [ V(i,j) for i in range(9) for j in range(9)]
    with open('zad_output.txt','w') as file :
        file.write(':- use_module(library(clpfd)).\n')
        file.write('solve([' + ', '.join(variables) + ']) :- \n')
        
        cs = domains(variables) + vertical() + horizontal() + squares() #TODO: too weak contraints, add something!
        for i,j,val in assigments:
            cs.append( '%s #= %d' % (V(i,j), val) )
        
        print_constraints(cs, 4, 70, file),
        file.write('')
        file.write('    labeling([ff], [' +  ', '.join(variables) + ']).' )
        file.write('\n')
        file.write(':- solve(X), write(X), nl.')       

if __name__ == "__main__":
    raw = 0
    triples = []
    with open('zad_input.txt','r') as file :
        inp = file.read()
    for x in inp.split('\n'):
        x = x.strip()
        if len(x) == 9:
            for i in range(9):
                if x[i] != '.':
                    triples.append( (raw,i,int(x[i])) ) 
            raw += 1          
    sudoku(triples)
    
"""
89.356.1.
3...1.49.
....2985.
9.7.6432.
.........
.6389.1.4
.3298....
.78.4....
.5.637.48

53..7....
6..195...
.98....6.
8...6...3
4..8.3..1
7...2...6
.6....28.
...419..5
....8..79

3.......1
4..386...
.....1.4.
6.924..3.
..3......
......719
........6
2.7...3..
"""    
