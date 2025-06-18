import random
from collections import deque

MOVES = {
        "L" : (0,-1),
        "R" : (0,1),
        "U" : (-1,0),
        "D" : (1,0),
    }
INF = 1000000000

class Maze :


    def __init__(self, maze : str) :
            '''
            G - meta
            S - start
            B - start/meta
            '''
            self.board = [[a for a in row] for row in maze.split('\n')][:-1]
            self.height = len(self.board)
            self.width = len(self.board[0])
            self.state_space = set()
            self.starts = set()
            self.goals = set()
            for row in range(self.height) :
                for col in range(self.width) :
                    match self.board[row][col] :
                        case 'G' :
                            self.goals.add((row,col))
                        case 'S' :
                            self.starts.add((row,col))
                        case 'B' :
                            self.starts.add((row,col))
                            self.goals.add((row,col))

    def hash_set(self, to_hash : set) :
        to_hash = list(to_hash)
        to_hash = sorted(to_hash)
        to_hash = '|'.join(['#'.join(str(a)) for a in to_hash])
        return to_hash

    def in_bounds(self, row : int, col : int) :
        return (row>=0 and row<self.height) and (col>=0 and col<self.width)

    def set_travel(self, starts : set, move : chr) -> set :
        next_positions = set()
        dy, dx = MOVES[move]
        for row, col in starts :
            next_row, next_col = row+dy,col+dx
            if  self.in_bounds(next_row,next_col) \
            and self.board[next_row][next_col] != '#' :
                next_positions.add((next_row,next_col))
            else :
                next_positions.add((row,col))
        return next_positions

    def phase_one(self, longest) :
        curr_starts = self.starts
        curr_path = ""
        for _ in range(longest) :
            if len(curr_starts) <= 2 :
                break
            
            min_starts = None
            min_move = ""
            for move in "URLD" :
                next_starts = self.set_travel(curr_starts, move)
                if min_starts is None or len(next_starts) < len(min_starts) :
                    min_starts = next_starts
                    min_move = move
            if len(min_starts) < len(curr_starts) :
                curr_path += min_move
                curr_starts = min_starts
            else :
                next_move = random.choice(['U','R','L','D'])
                curr_path += next_move
                curr_starts = self.set_travel(curr_starts, next_move)
        return curr_starts, curr_path

    def phase_two(self, init_starts, init_path) :
        q = deque()
        q.append((init_starts, init_path))
        self.state_space = set()
        while len(q) > 0 :
            curr_starts, curr_path = q.popleft()
            for move in "URLD" :
                next_path = curr_path + move
                next_starts = self.set_travel(curr_starts, move)
                if self.goals.issuperset(next_starts) :
                    return next_path
                elif len(next_path) < 150 \
                    and not (key := self.hash_set(next_starts)) in self.state_space :
                    self.state_space.add(key)
                    q.append((next_starts,next_path))
        return None

    def solve(self) :
        cnt = 110
        while True :
            cnt = 110 if cnt == 150 else cnt
            starts, path = self.phase_one(cnt)
            cnt += 1
            if len(starts) <= 2 :
                #print(cnt)
                result = self.phase_two(starts, path)
                if result != None :
                    return result
                else :
                    cnt = 110

if __name__ in "__main__" :
    with open("zad_input.txt",'r') as file :
        maze = Maze(file.read())
    with open("zad_output.txt",'w') as file :
        file.write(maze.solve())