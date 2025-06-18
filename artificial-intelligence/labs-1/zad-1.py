from collections import deque
import sys
'''
on_board - dla koordynatow postaci (x,y) sprawdza czy znajdują się w granicach planszy
occupied - dla podanych koordynatow figur podaje listę pól, które mogą "zbić",
wywolania dla 1 i 2 argumentow w zaleznosci od koloru
possible_moves - nazwa mowi sama za siebie, w przypadku zbicia wiezy - pat, gdy 
czarny krol nie ma ruchow to czy jest mat jest okreslone przez polozenie czarnego krola
uzywam klasycznego bfsa ( + odpowiednie warunki koncowe ), gdzie wierzcholkami sa stany planszy a krawedzie wyznacza
funkcja possible moves 
'''




moves_did = {}
on_board = lambda x,y : x >=0 and y >= 0 and x < 8 and y < 8

def occupied(king, rook=None) :
    king_fields = []
    rook_fields = []
    x, y = king
    for i in range(x-1, x+2) :
        king_fields.extend([(i,j) for j in range(y-1,y+2) if on_board(i,j)])
    if not rook is None :
        x, y = rook
        for i in range(0,8) :
            rook_fields.append((x,i))
            rook_fields.append((i,y))
    king_fields = list(filter(lambda x : x != king, king_fields))
    rook_fields = list(filter(lambda x : x != rook, rook_fields))
    return king_fields + rook_fields

def possible_moves(white_king, black_king, rook, color) :
    res = []
    match color :
        case 0 : #black
            dangerous_fields = occupied(king=white_king, rook=rook)
            x,y = black_king
            for i in range(x-1,x+2) :
                for j in range (y-1,y+2) :
                    if (on_board(i,j) 
                        and not (i,j) in dangerous_fields
                        and (i,j) != black_king
                        )  :
                        move = (white_king, (i,j), rook)
                        res.append(move)
            if res == [] :
                # jesli nie ma dozwolnych ruchow sprawdz czy czarny krol artakowany
                return black_king in dangerous_fields
        case 1 : #white
            if black_king == rook :
                # sprawdz czy wieza zyje
                return False
            dangerous_fields = occupied(king=black_king)
            x,y = white_king
            for i in range(x-1,x+2) :
                for j in range(y-1,y+2) :
                    if (on_board(i,j) 
                        and not (i,j) in dangerous_fields
                        and (i,j) != white_king) :
                        move = ((i,j), black_king, rook)
                        res.append(move)
            x,y = rook
            for i in range(0,8) :
                move_1 = (white_king, black_king, (x,i))
                move_2 = (white_king, black_king, (i,y))
                res.append(move_1)
                res.append(move_2)
    return res

def bfs(white_king, black_king, rook, color, previous_moves) :
    global move_queue
    moves_or_mate = possible_moves(white_king, black_king, rook, color)
    match moves_or_mate:
        case True :
            return (previous_moves, True)
        case False :
            pass
        case _ :
            for move in moves_or_mate :
                new_move = move + (color^1, [*previous_moves,move])
                if not move + (color^1,) in moves_did :
                    moves_did[move+(color^1,)] = True
                    move_queue.append(new_move)
    return ([], False)

def parse_input(line) :
    def parse_figure(text) :
        x = ord(text[0]) - ord('a')
        y = int(text[1]) - 1
        return (x,y)
    color, white_king, rook, black_king = line.split(' ')
    color = 0 if color == "black" else 1
    white_king = parse_figure(white_king)
    black_king = parse_figure(black_king)
    rook = parse_figure(rook)
    return (white_king, black_king, rook, color)

def pretty_print(white_king, black_king, rook) :
    print(' ',end='')
    for x in range(ord('A'), ord('H')+1) :
        print(f"{chr(x)} ",end='')
    print()
    print(f"┌{'─┬'*7}─┐")
    for row in range(7,-1,-1) :
        for col in range(0,8) :
            tile = ' '
            if (col,row) == white_king :
                tile = '♚'
            if (col,row) == black_king :
                tile = '♔'
            if (col,row) == rook :
                tile = '♜'
            print(f'│{tile}',end='')
        print('│',row+1)
        if row > 0 :
            print(f"├{'─┼'*7}─┤")

            

    print(f"└{'─┴'*7}─┘")


def main(data) :
    global move_queue
    init_move = parse_input(data) + ([],)
    move_queue = deque([init_move])
    game_status = ([], False)
    while move_queue :
        game_status = bfs(*move_queue.popleft())
        if game_status[1] :
            break
    return game_status
    





if __name__ in "__main__"  :
    with open("zad1_input.txt",'r',encoding='utf-8') as file :
        for line in file :
            moves, mate = main(line)
            with open("zad1_output.txt",'w',encoding="utf-8") as result :
                if not mate :
                    result.write("INF")
                else :
                    if __debug__ :
                        for move in moves :
                            print(*move)
                            pretty_print(*move)
                    result.write(str(len(moves)))
                
            
            