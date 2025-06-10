Specyfikacja projektu :

Biblioteki potrzebne do uruchomienia aplikacji :
sys, os, copy, random 
( wszystkie należą do SPL, więc wystarczy zainstalować pythona ) 

Wersja Pythona, na której tworzony był projekt :
Python 3.11.5

Pliki zawarte w projekcie :
- main.py 
główny plik, którego uruchomienie skutkuje rozpoczęciem gry w terminalu

- board.txt
zawiera opis planszy

- Dictionaries
	- PL (słownik słów w jęz polskim, które są dopuszczalne w grach)
	- EN ( - // - angielskim - // -)

Zasady gry : 

Gra przeznaczona jest dla 2 - 4 graczy ( tak jak w oryginalnej wersji )
Plansza jest zaprojektowana tak, jak w oryginalnej wersji z legendą :
D L - double letter ( podwójna litera )
T L - triple letter ( potrójna litera )
D W - double word ( podwójne słowo )
T W - triple word ( potrójne słowo )

W worku znajduje się +-100 płytek 
( ta liczba może być nieznacznie większa lub mniejsza ze względu na dynamiczne obliczanie worka )

Rozgrywka : 

Pierwszy gracz tworzy z dwóch lub więcej płytek słowo i układa je na planszy poziomo lub pionowo, w taki sposób, by jedna płytka znalazła się w centralnym polu z gwiazdką. Układanie słów na skos nie jest dozwolone. Na koniec każdej swojej kolejki gracz podaje wynik jaki uzyskał z ułożenia słowa, zapisuje go i dobiera tyle płytek, żeby na stojaku miał zawsze siedem. Drugi gracz może dołożyć jedną lub kilka płytek do już położonych na planszy i utworzyć nowe słowo o długości co najmniej dwóch liter. Gracz może też zdecydować się na opuszczenie swojej kolejki lub wymianę płytek.

Jeśli litery stykają się z innymi literami z sąsiednich rzędów, muszą z nimi tworzyć pełne słowo, tak jak w klasycznej krzyżówce. Podczas jednej kolejki nie można dokładać płytek do różnych słów ani tworzyć nowych słów w różnych częściach planszy.

Gracz otrzymuje punkty za wszystkie słowa utworzone lub zmodyfikowane w wyniku swojego ruchu. Na planszy znajdują się specjalne pola premiowe – podwójna, potrójna premia literowa oraz premia słowna. Każdy gracz może zrezygnować ze swojej kolejki na rzecz wymiany płytek. Zdejmuje je ze stojaka tak, żeby nikt ich nie widział, losuje ile chce, a swoje wrzuca do woreczka. Można też zrezygnować z kolejki. Gra kończy się, kiedy wszyscy gracze dwa razy z rzędu opuszczą swoją kolejkę.

Poruszanie się po rozgrywce i to co użytkownik powinien zrobić jest opisane na każdym etapie rozgrywki.

Uruchomienie projektu :
Należy uruchomic folder PROJEKT w terminalu bez zmieniania struktury rozmieszczenia plików, oraz
uruchomić dokument main.py komendą 
py main.py 

Źródła :
	- zasady : pisupisu.pl
	- słownik 'PL' : sjp.pl
	- słownik 'EN' : Collins Scrabble Words (2019)
