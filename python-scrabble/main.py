import os
from copy import deepcopy
from random import shuffle
import sys
# --------------------------- specyfikacja ----------------- #
dictionaries_path = "Dictionaries"
default_dict_path = "Dictionaries\PL.txt"
default_dict_name = "PL"
default_player_number = 2
default_bag_size = 100
board_path = "Board.txt"
board_size = 15
main_game = False
# -------------------- stale (czcionki, kolory) ------------ #
BOLD = '\033[1m'
END = '\033[0m'
REVERSE = "\033[;7m"
RESET = "\033[0;0m"
# ---------------------------------------------------------- #
def clear_ter() :
   os.system('cls' if os.name == 'nt' else 'clear')

class Lang_list :
# konstruktor przyjmujący ścieżkę do folderu w którym znajdują się słowniki
  def __init__(self,path = dictionaries_path):
    self.lang_list = []
    for x in os.listdir(path):
      if str(x).endswith('.txt') :
        self.lang_list.append(x[:-4])
# pokazuje jakie języki są dostępne dla użytkownika  
  def show_list(self):
    print("Lista dostępnych języków : ")
    for x in self.lang_list :
      print("- \"" + x +"\"")

class Dictionary :
# konstruktor przyjmujący ścieżkę do słownika (domyślnie jęz polski) oraz nazwę słownika  
  def __init__(self,path = default_dict_path, name = default_dict_name):
      self.lang_name = name
      self.word_list = []
      self.set_dict(path)
# przyjmuje ścieżkę do słownika po czym tworzy listę słów na podstawie pliku  
  def set_dict(self,dict_path) : 
      dict_file = open(dict_path,"r",encoding='utf-8')
      self.word_list = [x.strip() for x in dict_file]
      dict_file.close()
# sprawdza czy podane słowo znajduje się w słowniku   
  def is_valid(self,word):
     return word.strip() in self.word_list

class Tile_bag :
# konstruktor przyjmujący słownik oraz rozmiar (domyślnie 100)
   def __init__(self,dict : Dictionary,bag_size = default_bag_size) :
         self.content = []                                           # lista par (litera,cnt)
         self.tile_stack = []                                        # lista liter (pomieszane)
         self.point_list = []                                        # lista par (litera,punkty)
         dict_size = 0
         assoc_list = [[chr(x),0] for x in range(0,1000)]
         for word in dict.word_list :
            for letter in word :
               assoc_list[ord(letter)][1] += 1
               dict_size += 1
         for a,b in assoc_list :
            if b == 0 :
               continue
            cnt = int(b*bag_size/dict_size)
            if cnt <= 1 and b > 100:
               cnt += 1
            if cnt > 0 :
               self.content.append((a,cnt))
         for (letter,cnt) in self.content :
            self.point_list.append((letter,int((5/cnt)+1)))        
         self.shuffle_bag()
# miesza worek tworzac przy tym tile_stack
   def shuffle_bag(self):
      self.tile_stack = []
      for x in self.content :
         for i in range(0,x[1]):
            self.tile_stack.append(x[0])
      shuffle(self.tile_stack)
# sprawdza pustosc worka
   def is_empty(self):
      return self.tile_stack == []
# zwraca plytke na gorze worka, uktualizuje content, miesza worek
   def draw_tile(self) :
      if self.is_empty() :
         return ""
      tile = self.tile_stack[0]
      new_bag = []
      for x in self.content:
         if x[0] == tile :
            new_bag.append((x[0],x[1]-1))
         else :
            new_bag.append((x[0],x[1]))
      self.content = new_bag
      self.shuffle_bag()
      return tile
# dodaje plytke zadana przez parametr do worka, aktualizuje content, miesza worek
   def add_tile(self,tile) :
      new_bag = []
      for x in self.content:
         if x[0] == tile :
            new_bag.append((x[0],x[1]+1))
         else :
            new_bag.append((x[0],x[1]))
      self.content = new_bag
      self.shuffle_bag()
# zwraca ile punktow za dana plytke
   def get_score(self,tile) : 
      for l,pkt in self.point_list :
          if l == tile :
             return pkt
      return 0  
# ile plytek jeszcze zostalo
   def size(self) :
      return len(self.tile_stack)

class Board : 
   def __init__(self,path = board_path) : 
      board_file = open(path,"r")
      self.tiles = []
      for row in board_file :
         row = row.strip().split(',')
         self.tiles.append(row)
# pokazuje plansze
   def show(self) :
      def writerow(cross = '┿',en = "┥"): 
         for i in range(0,62):
            if i%4 == 2 :
               print(cross,end = '')
            else :
               print('━',end = '')
         print(en)
      print(" ╲│ ",end='')
      for i in range(0,board_size) :
         letter = chr(ord('A') + i)
         print(letter + " │ ",end = '')
      print("")
      writerow()
      for i in range(0,board_size) :
          letter = str(i + 1)
          if i + 1 < 10 :
           print(letter + " │",end = '')
          else :
           print(letter + "│",end = '')
          for cell in self.tiles[i] :
               if len(cell) == 3 :
                  print(cell+'│',end = '')
               elif '_' in cell :
                  print(' _ │',end = '')
               else :
                  sys.stdout.write(REVERSE)
                  print(f"{BOLD} {cell} {END}",end = '')
                  sys.stdout.write(RESET)
                  print('│',end = '')

          print('')
          if i + 1 < board_size :
            writerow()
          else :
             writerow('┷','┙')
# daje literke na danej pozycji lub zwraca False jesli plytka jest pusta
   def get_tile(self,x,y) :
       if self.tiles[y][x] == '_' or not len(self.tiles[y][x]) == 1 :
          return False
       return self.tiles[y][x]  
# kladzie slowo na planszy x (A,B ... O), y (1,2 ... 15), kierunek (Prawo,Dol)
# zwraca boola czy udalo sie polozyc slowo oraz punkty jakie by zdobyl ten ruch
   def place(self,word,x,y,dir,bag : Tile_bag) :
       x = int(ord(x) - ord('A'))
       y = int(y) - 1
       word_mult = 1
       word_points = 0
       connected = False
       def connect(i,j) :
          mv = [(-1,0),(1,0),(0,-1),(0,1),(0,0)]
          for a,b in mv :
             curr_pos = (i+a,j+b) 
             if curr_pos[0] < 0 or curr_pos[1] < 0 or curr_pos[0] >= board_size or curr_pos[1] >= board_size :
                continue
             curr_tile = self.tiles[curr_pos[0]][curr_pos[1]]
             if not('_' in curr_tile or 'L' in curr_tile or 'W' in curr_tile) :
                return True
          return i == 7 and j == 7       
       match dir:
          case 'P' :
             if x + len(word) - 1 >= board_size :
                return (False,0)
             iter = x 
             for letter in word : 
                if not ( self.get_tile(iter,y) == False or self.get_tile(iter,y) == letter ) :
                   return (False,0) 
                if connected == False :
                   connected = connect(y,iter)
                iter += 1
             iter = x
             for letter in word :
                match self.tiles[y][iter].replace(' ','') :
                   case '_' :
                      word_points += bag.get_score(letter)
                   case 'DL':
                      word_points += 2 * bag.get_score(letter)
                   case 'TL':
                      word_points += 3 * bag.get_score(letter)
                   case 'DW':
                      word_mult *= 2
                      word_points += bag.get_score(letter)
                   case 'TW':
                      word_mult *= 3
                      word_points += bag.get_score(letter)
                self.tiles[y][iter] = letter
                iter += 1
             return (connected,word_points*word_mult)              
          case 'D' :
             if y + len(word) - 1 >= board_size :
                return (False,0)
             iter = y
             for letter in word :
                if not (self.get_tile(x,iter) == False or self.get_tile(x,iter) == letter ) :
                  return (False,0)
                if connected == False :
                   connected = connect(iter,x)
                iter += 1
             iter = y
             for letter in word :
                match self.tiles[iter][x].replace(' ','') :
                   case '_' :
                      word_points += bag.get_score(letter)
                   case 'DL':
                      word_points += 2 * bag.get_score(letter)
                   case 'TL':
                      word_points += 3 * bag.get_score(letter)
                   case 'DW':
                      word_mult *= 2
                      word_points += bag.get_score(letter)
                   case 'TW':
                      word_mult *= 3
                      word_points += bag.get_score(letter)
                self.tiles[iter][x] = letter
                iter += 1
             return (connected,word_points*word_mult)
       raise TypeError
# weryfikuje poprawnosc planszy na podstawie slownika
   def verify(self, dict : Dictionary):         
         # weryfikuje czy plytka tworzy same poprawne slowa
         def verify_tile(x : int, y : int) : 
             curr_word = self.get_tile(x,y)
             if curr_word == False :
                return True
             iter = x + 1
             while iter < board_size :
               curr_tile = self.get_tile(iter,y)
               if curr_tile == False :
                  break
               else :
                  curr_word += curr_tile
                  iter += 1
             iter = x - 1
             while iter >= 0 :
               curr_tile = self.get_tile(iter,y)
               if curr_tile == False :
                  break
               else :
                  curr_word = curr_tile + curr_word
                  iter -= 1
             word_x = curr_word
             curr_word = self.get_tile(x,y)
             iter = y + 1
             while iter < board_size :
                curr_tile = self.get_tile(x,iter)
                if curr_tile == False :
                   break
                else :
                   curr_word += curr_tile
                   iter += 1
             iter = y - 1
             while iter >= 0 : 
                curr_tile = self.get_tile(x,iter) 
                if curr_tile == False :
                   break
                else :
                   curr_word = curr_tile + curr_word
                   iter -= 1
             word_y = curr_word
             return (len(word_x) == 1 or dict.is_valid(word_x)) and (len(word_y) == 1 or dict.is_valid(word_y))
         for i in range(0,board_size) :
            for j in range(0,board_size):
               if verify_tile(i,j) == False :
                  return False
         return True
# daje liste liter potrzebnych do wykonania ruchu
   def tiles_to_place(self,word,x,y,dir) :
      curr_list = []
      x = int(ord(x) - ord('A'))
      y = int(y) - 1
      match dir :
         case 'P' :
            iter = x
            for letter in word :
               if self.tiles[y][iter].replace(' ','') in ['_','TW','DW','TL','DL'] :
                     curr_list.append(letter)
               iter += 1
         case 'D' :
            iter = y
            for letter in word :
               if self.tiles[iter][x].replace(' ' ,'') in ['_','TW','DW','TL','DL'] :
                     curr_list.append(letter) 
               iter += 1
      return curr_list

class Player :
   def __init__(self,name) :
      self.name = name
      self.inventory = [] 
      self.score = 0
   def show_eq(self, bag : Tile_bag) :
      print("Eq gracza :")
      for x in sorted(self.inventory) :
         print(x,end = ' ')
      print("")
      for x in sorted(self.inventory) :
         print(bag.get_score(x),end = ' ')
      print("")
   def show_score(self,char = '\n') :
      print("Aktualny wynik : " + str(self.score), end = char)

class Menu :
   def __init__(self) :
      self.dict = Dictionary()
      self.player_num = 2
      self.name_list = ['Gracz 1','Gracz 2']
   def show_menu(self) :
       clear_ter()
       print("Q - Wyjście")
       print("Ustawienia :")
       print("1. Aktualny język : " + self.dict.lang_name)
       print("2. Liczba graczy : " + str(self.player_num))
       print("3. Lista graczy : ")
       for x in self.name_list :
          print(" - " + x)
       action = input("Nacisnij numer ustawienia aby je zmienic, 'Q' aby wyjść lub cokolwiek innego aby zaczac gre\n")
       self.response(action)
   def response(self,action) : 
      clear_ter()
      match action.strip():
         case  '1' : 
            #set dictionary 
            lang_list = Lang_list()
            lang_list.show_list()
            chosen_lang = input("Podaj język w którym chcesz grać : \n").strip()
            while not chosen_lang in lang_list.lang_list :
               clear_ter()
               print("Wybrano nieistniejacy jezyk ! ")
               lang_list.show_list()
               chosen_lang = input("Podaj jezyk w ktorym chcesz grac : \n").strip()
            file_path = dictionaries_path + "\\" + chosen_lang + '.txt'
            self.dict = Dictionary(file_path,chosen_lang)
            input("Ustawiono jezyk na : " + chosen_lang + "\nNaciśnij enter aby kontynuowac")
            self.show_menu()
         case  '2' :
            # set player count and adjust players' name list size
            new_cnt = input("Ustaw liczbę graczy (2-4) : \n").strip()
            while not new_cnt in ['2','3','4'] :
               clear_ter()
               print("Wybierz liczbę od 2 do 4 !")
               new_cnt = input("Wybierz liczbę graczy : \n")
            new_cnt = int(new_cnt)
            self.player_num = new_cnt
            while len(self.name_list) < new_cnt :
               self.name_list.append("Gracz " + str(len(self.name_list)+1))
            while len(self.name_list) > new_cnt :
               self.name_list.pop()
            print('Pomyslnie ustawiono liczbe graczy na : ' + str(new_cnt))
            input('Nacisnij enter aby kontynuowac \n')
            self.show_menu()
         case  '3' : 
            print('Aktualna lista graczy : ')
            for x in range(0,len(self.name_list)):
               print(str(x+1)+'. '+self.name_list[x])
            numer = input("Wybierz numer gracza, którego nick chcesz zmienic lub cokolwiek innego aby wrocic do menu :\n").strip()
            if not numer in ['1','2','3','4'] or int(numer) > len(self.name_list) :
               self.show_menu()
            numer = int(numer)
            clear_ter()
            new_name = input("Wpisz nowy nick dla gracza (dlugosc nicku miedzy 3 a 20): " + self.name_list[numer-1] + "\n")
            while len(new_name) < 3 or len(new_name) > 20 :
               clear_ter()
               new_name = input("Wpisz nowy nick dla gracza (dlugosc nicku miedzy 3 a 20): " + self.name_list[numer-1] + "\n")
            new_name = new_name.strip()
            self.name_list[numer-1] = new_name
            self.response('3')
         case  'Q' :
            exit()
         case _ : 
            # start game
            main_game = Game(self.name_list,self.player_num,self.dict)
            main_game.move_init()

class Game :
   def __init__(self, player_list, player_num, dict : Dictionary) :
      self.dict = dict
      self.bag = Tile_bag(dict)
      self.board = Board()
      self.player_num = player_num
      self.player_list = [Player(x) for x in player_list]
      for x in range(0,len(self.player_list)) :
         for i in range(0,8) :
            self.player_list[x].inventory.append(self.bag.draw_tile())
      self.move_cnt = 0
      self.pass_streak = 0
# odpowiedz na ruch 
   def move_response(self,move_type) :
      clear_ter()
      match move_type :
# opis procesu passowania ruchu
         case 'P' :
            confirmation = input("Czy na pewno chcesz spassowac ruch ? (T/_)\n").strip()
            if confirmation == 'T' :
               self.move_cnt += 1
               self.pass_streak += 1
               if self.pass_streak >= self.player_num * 2 :
                  input("Osiągnięto limit passów bez położenia słowa ! \nNaciśnij enter aby zobaczyć podsumowanie")
                  return self.end_screen()
            else :
               clear_ter()
               input("Anulowano ruch, nacisnij enter aby wrocic do menu ruchu")
               return self.move_init()
# opis wymieniania liter 
         case 'W' :
      # sprawdzam czy worek jest pusty i zawracam gracza do menu
            if self.bag.size() == 0 :
               input("Worek jest pusty ! Naciśnij enter aby wrócić do menu wyboru ruchu")
               return self.move_init()
      # overview ruchu      
            print("Ilość płytek w worku : ",self.bag.size())
            curr_player = self.player_list[self.move_cnt%self.player_num]
            curr_player.show_eq(self.bag)
            num_list = input("Podaj listę numerów płytek które chcesz wymienić oddzielone spacją\nlub 'Q' nic jesli chcesz wrocic do menu\n").strip()
      # anulowanie ruchu powrot do menu      
            if num_list == 'Q' :
               return self.move_init()
      # sprawdzenie poprawnosci danych       
            num_list = num_list.split(' ')
            if len(num_list) > self.bag.size() :
                  return self.move_response('W')
            for x in num_list :
               if not x in ['1','2','3','4','5','6','7','8'] :
                  return self.move_response('W')
               if int(x) > len(curr_player.inventory) :
                  return self.move_response('W')       
      # wykonanie zamiany     
            tile_change = []
            for i in range(0,len(curr_player.inventory)) :
               if str(i+1) in num_list :
                  tile_change.append(curr_player.inventory[i])
                  new_tile = self.bag.draw_tile()
                  print(curr_player.inventory[i] + ' -> ' + new_tile)
                  curr_player.inventory[i] = new_tile
            for x in tile_change :
               self.bag.add_tile(x)
            curr_player.show_eq(self.bag)
            input("\nNaciśnij enter aby kontynuować")
            # nie zmieniam pass streak
            self.move_cnt += 1
# opis ruchu kladacego slowo
         case 'I' : 
            self.board.show()
            curr_player = self.player_list[self.move_cnt%self.player_num]
            curr_player.show_eq(self.bag)
   # wpisanie danych dla ruchu
            word = input("Podaj słowo które chcesz położyć : ").strip()
            x = input("Podaj kolumnę (A - O) : ").strip()
            y = input("Podaj wiersz (1 - 15) : ").strip()
            dir = input("Podaj kierunek położenia słowa (P - prawo, D - dół) : ").strip()
            confirmation = input("Zatwierdź ruch / Wróć do menu ruchu (T/_) : ")
            if not confirmation == 'T' :
               return self.move_init()
   # weryfikacja wpisanych danych
            if not (
            x in ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O'] and
            y in ['1','2','3','4','5','6','7','8','9','10','11','12','13','14','15'] and
            dir in ['P','D'] ) :
               input("Podano dane w nieprawidłowym formacie ! Naciśnij enter aby spróbować ponownie")
               return self.move_response('I')
            move = Move(self.board,curr_player,word,x,y,dir,self.bag,self.dict)
   #  sprawdzenie czy plansza jest pusta i czy ruch przechodzi przez srodek
            if self.board.tiles == Board().tiles : 
               center = False 
               pos_y = int(y) - 1
               pos_x = int(ord(x) - ord('A'))
               match dir :
                  case 'P' :
                     for i in range(0,len(word)) :
                        if not pos_y == 7 :
                           break
                        if pos_x + i == 7 : 
                           center = True
                  case 'D' :
                     for i in range(0,len(word)) :
                        if not pos_x == 7 :
                           break 
                        if pos_y + i == 7 :
                           center = True
               if center == False :
                  input("Pierwszy ruch musi przechodzić przez środek ! Naciśnij enter aby spróbować ponownie")
                  return self.move_response('I')
   #  sprawdza poprawnosc ruchu
            match move.make_move() : 
               # ruch nieprawidlowy 
               case False, info, _ : 
                  match info[0] :
                     case 'len' :
                        input("Słowo powinno mieć długość >=2")
                     case 'dict' :
                        input('Ruch psuje poprawność planszy')
                     case 'eq' :
                        input('Nie masz liter potrzebnych do wykonania ruchu')
                  return self.move_response('I')
               # wykonuje ruch
               case True,drawn_tiles, score : 
                  print("Wykonano ruch ! \nWylosowano : ",end = ' ')
                  for x in drawn_tiles : 
                     print(x,end = ' ')
                  print("")
                  curr_player.show_score('')
                  print(' ( + '+ str(score) +' )')
                  input("Naciśnij enter aby przejść dalej")
                  self.move_cnt += 1
                  self.pass_streak = 0      
# next move initiation
      clear_ter()
      input("Teraz bedzie ruch gracza : " + self.player_list[self.move_cnt%self.player_num].name + " nacisnij enter aby kontynuowac")
      self.move_init()
# ekran wyboru ruchu
   def move_init(self) :
         clear_ter()
         self.board.show()
         curr_player = self.player_list[self.move_cnt%self.player_num]
         curr_player.inventory = sorted(curr_player.inventory)
         print("Ilość passów bez położenia słowa na planszy :",self.pass_streak,' / ',self.player_num*2)
         print("Ruch gracza : " + curr_player.name) 
         curr_player.show_eq(self.bag)    
         curr_player.show_score()  
         print("Wybierz typ ruchu :")
         print("W - wymien literki ")
         print("P - pass")
         print("I - połóż słowo")
         action = input("").strip()
         while not action in ['W','P','I'] :
            action = input("\033[F").strip()
         self.move_response(action)
# ekran końcowy
   def end_screen(self) :
      def get_leaderboard(player_list) :
            new_list = sorted(player_list, key=lambda x : x.score,reverse = True)
            return new_list
      clear_ter()
      print("Gra zakończona ! \n Tabela wyników :")
      leaderboard = get_leaderboard(self.player_list)
      place = 0
      prev_score = -1
      for x in leaderboard :
         if not prev_score == x.score :
            place += 1
            prev_score = x.score
         if not place == 1 :   
            print(' ',place,end = '. ')
         else :
            print('🏆',end = '. ')
         print(x.name,'    ',x.score)
      input("Nacisnij enter aby zakonczyc gre")
      exit()

class Move :
   def __init__(self, board, player : Player, word, x, y, dir, bag : Tile_bag, dict : Dictionary) :
      self.board = board
      self.player = player
      self.word = word
      self.dir = dir
      self.x = x
      self.y = y
      self.bag = bag
      self.dict = dict
   # tworzy wirtualna kopie planszy i sprawdza czy ruch jest mozliwy do wykonania i czy nie zaburza poprawnosci planszy
   def board_test(self) :
       new_board = deepcopy(self.board)
       if new_board.place(self.word,self.x,self.y,self.dir,self.bag)[0] == False :
          return False 
       elif new_board.verify(self.dict) == False :
          return False
       return True
   # sprawdza czy gracz ma plytki potrzebne do polozenia slowa zakladajac ze ruch w ogolnosci jest poprawny
   def check_eq(self) : 
       tiles = self.board.tiles_to_place(self.word,self.x,self.y,self.dir)
       temp_eq = [x for x in self.player.inventory]
       for letter in tiles :
          guard = False
          for x in range(0,len(temp_eq)) :
              if temp_eq[x] == letter :
                 temp_eq[x] = '-'
                 guard = True
                 break
          if guard == False :
             return (False,[])
       return (True,tiles)           
   # sprawdza legalnosc ruchu i w przypadku pozytywnej weryfikacji wykonuje go
   def make_move(self) :
       eq = self.check_eq()
       if len(self.word)>1 and self.board_test() and eq[0] == True :
# położenie słowa na planszy (bool, dobrane plytki, score)
         x = self.board.place(self.word,self.x,self.y,self.dir,self.bag)
         curr_score = x[1]
         self.player.score += curr_score
# korekta worka gracza
         tiles = eq[1]
         temp_eq = [x for x in self.player.inventory]
         for letter in tiles :
          for x in range(0,len(temp_eq)) :
              if temp_eq[x] == letter :
                   temp_eq[x] = '-'
                   break
         self.player.inventory = []
         for letter in temp_eq :
            if not letter == '-' :
               self.player.inventory.append(letter)
         drawn_tiles = []
         while len(self.player.inventory) < 8 and not self.bag.is_empty() :
               tile = self.bag.draw_tile()
               drawn_tiles.append(tile)
               self.player.inventory.append(tile)
         return (True,drawn_tiles, curr_score)
# srawdzenie co poszlo nie tak         
       if eq[0] == False :
         return (False,['eq'],0)
       if len(self.word) <= 1 :
          return (False,['len'],0)
       return (False,['dict'],0)

if __name__ == "__main__":
   menu = Menu()
   menu.show_menu()





