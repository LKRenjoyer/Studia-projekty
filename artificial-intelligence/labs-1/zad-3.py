import random
import time
from itertools import product, combinations
from sortedcontainers import SortedSet
'''
obiekt rules - 
    kluczowy jest eval, ktory ewaluuje "moc" ręki gracza
    compare - zwraca ktora reka jest wygrywajaca, wiadomo, ze w przypadku
    remisu zawsze wygra figurant, bowiem jego karty sa mocniejsze
get_hand - wybiera rękę dla gracza na podstawie talii 
play - symuluje rozgrywke, przyjmuje 2 talie losuje graczom karty i zwraca wynik
stats_for_set - symuluje SAMPLE_SIZE gier i zwraca odpowiednio punkty - tą funkcją szacujemy
szanse obu graczy w starciu ze soba
check - dla odpowiedniego rozmiaru sprawdza wszystkie mozliwe talie blotkarza o tym rozmiarze
i informuje o znalezieniu (bądź nie) talii z danym rozmiarem pokonujacej figuranta
'''


SAMPLE_SIZE = 100
figure_types = ['J','Q','K','A']
blotki = range(2,11)
colors = ['pik','kier','karo','trefl']

full_blot_set = list(product(blotki, colors))
full_fig_set = list(product(figure_types, colors))


class Rules:
    def __init__(self) :
        combinations = [
            'high-card',
            'pair',
            'two-pairs',
            'triple',
            'strit',
            'color',
            'full',
            'kareta',
            'poker',
        ]
        self.worth = dict(zip(combinations, range(9)))
    def eval(self, hand : list[tuple[str,str]], hand_type) :
        colors = {}
        values = {}
        for val, kol in hand :
            values[val] = values.get(val,0) + 1
            colors[kol] = colors.get(kol,0) + 1
        pair_cnt = len(list(filter(lambda x : x, [x==2 for x in values.values()])))
        triple = any([x==3 for x in values.values()])
        four_of_a_kind = any([x==4 for x in values.values()])
        color = len(colors) == 1
        if hand_type == 'blot' :
            values = sorted(int(x) for x in values.keys())
            consecutive = len(values) == 5 and values[0] + 4 == values[-1]
        else :
            consecutive = False
        if consecutive and color :
            return "poker"
        if four_of_a_kind :
            return "kareta"
        if triple and pair_cnt :
            return "full"
        if color :
            return "color"
        if consecutive :
            return "strit"
        if triple :
            return "triple"
        if pair_cnt == 2 :
            return "two-pairs"
        if pair_cnt == 1 :
            return "pair"
        return "high-card"
    def compare(self, blot_hand, fig_hand) :
        eval_1 = self.worth[self.eval(blot_hand,"blot")]
        eval_2 = self.worth[self.eval(fig_hand,"fig")]
        if eval_1 > eval_2 :
            return "first"
        if eval_1 == eval_2 :
            return "tie"
        if eval_1 < eval_2 :
            return "second"

def get_hand(card_set, hand_size=5) :
    return random.sample(card_set, hand_size)

def play(blot_set, fig_set, rules : Rules) :
    blot_h = get_hand(blot_set)
    fig_h = get_hand(fig_set)
    if rules.compare(blot_h, fig_h) in ["tie","second"] :
        return (0,1)
    else :
        return (1,0)

def stats_for_sets(blot_set, fig_set, sample_size = SAMPLE_SIZE) :
    rules = Rules()
    b_score = 0 
    f_score = 0
    for _ in range(sample_size) :
        b,f = play(blot_set, fig_set, rules)
        b_score += b
        f_score += f
    return b_score, f_score

def check(size) :
    possible_sets = combinations(full_blot_set, size)
    for curr_set in possible_sets :
        blot,fig = stats_for_sets(curr_set, full_fig_set)
        #print(blot,' ',fig,' ', size)
        if blot > fig :
            print(f"Found for size : {size}")
            print(f"Blotkarz : {blot}")
            print(f"Figurant : {fig}")
            return curr_set

    print(f"NOT found for size : {size}")
    return False

if __name__ in "__main__" :
    bl_score, fig_score = stats_for_sets(full_blot_set,full_fig_set,10000)
    print("Caly set :")
    print(f"Blotkarz : {bl_score}")
    print(f"Figurant : {fig_score}")
    for x in range(5,15,1) :
        winning_set = check(x)
        if winning_set :
            for card in winning_set :
                print(card)
