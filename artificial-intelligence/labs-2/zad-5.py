import time
from collections import deque
from queue import PriorityQueue
import random

MOVES = {
        "L" : (0,-1),
        "R" : (0,1),
        "U" : (-1,0),
        "D" : (1,0),
    }
INF = 1000000000
STOPIEN = 3
def measure(fun) :
    def wrapper(*args, **kwargs) :
        st = time.time()
        v = fun(*args, **kwargs)
        print(f"Executed in {time.time() - st}")
        return v
    return wrapper


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
            self.state_space = {}
            self.starts = set()
            self.goals = set()
            self.goal_dist = {}
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
            for goal in self.goals :
                self.goal_dist[goal] = self.flood(goal)
                #print(self.goal_dist[goal])

    def flood(self, start) :
        row, col = start
        init_board = [[INF for _ in range(self.width)] for _ in range(self.height)]
        init_board[row][col] = 0
        q = deque()
        q.append((row,col))
        while len(q) > 0 :
            row, col = q.popleft()
            for move in "URLD" :
                dy, dx = MOVES[move]
                next_row = row+dy
                next_col = col+dx
                if self.in_bounds(next_row, next_col) \
                    and self.board[next_row][next_col] != '#' \
                    and init_board[next_row][next_col] == INF :
                    init_board[next_row][next_col] = 1 + init_board[row][col]
                    q.append((next_row,next_col))
        return init_board

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

    def heuristic(self, positions, param = 1) :
        res = 0
        for row, col in positions :
            min_d = INF
            for goal in self.goals :
                r,c = goal
                #min_d = min(min_d, abs(c-col)+abs(r-row))
                min_d = min(min_d, self.goal_dist[goal][row][col])
            res = max(res,min_d)
        return res * param

    def phase_one(self, longest, limit) :
        curr_starts = self.starts
        curr_path = ""
        for _ in range(longest) :
            if len(curr_starts) <= limit :
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
        if len(curr_starts) <= limit :
            return curr_starts, curr_path
        else : 
            return None,None

    
    def phase_two(self, init_starts, init_path, param) :
        self.state_space = {}
        q = PriorityQueue()
        q.put((self.heuristic(init_starts, param),(init_starts, init_path)))
        while not q.empty() :
            _, (curr_starts, curr_path) = q.get()
            
            for move in "URLD" :
                next_path = curr_path + move
                next_starts = self.set_travel(curr_starts, move)
                if self.goals.issuperset(next_starts) :
                    return next_path

                else :
                    if (key := (self.hash_set(next_starts))) in self.state_space :
                        if self.state_space[key] > len(next_path) :
                            self.state_space[key] = len(next_path)
                            q.put((len(next_path) + self.heuristic(next_starts),(next_starts,next_path)))
                    elif not (key := (self.hash_set(next_starts))) in self.state_space :
                        self.state_space[key] = len(next_path)
                        q.put((len(next_path) + self.heuristic(next_starts, param),(next_starts,next_path)))
        return None

    @measure
    def solve(self, uncertainty, h_param) :
        for l_1 in range(0,100) :
            curr_starts, curr_path = self.phase_one(l_1, uncertainty)
            if curr_starts is None :
                continue
            else :
                result = self.phase_two(curr_starts,curr_path, h_param)
                if result is None :
                    continue
                else :
                    return result
        return self.solve(uncertainty+1, h_param)
    

    
    
if __name__ in "__main__" :
    with open("zad_input.txt",'r') as file :
        maze = Maze(file.read())
    with open("zad_output.txt",'w') as file :
        opt_path = maze.solve(2, 2)
        file.write(opt_path)
        
