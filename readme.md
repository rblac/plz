# plzerror
## english
A toy [PL/0](https://en.wikipedia.org/wiki/PL/0) tree-walk interpreter, made based on the [second part](https://craftinginterpreters.com/a-tree-walk-interpreter.html) of Robert Nystrom's wonderful book, [Crafting Interpreters](https://craftinginterpreters.com).

Written by me(Roch Błachut) as the grading assignment for my 2023-2024 Programming Languages class.

The [`pl0.ebnf`](./pl0.ebnf) file contains an EBNF description of PL/0's grammar, which I ripped straight from Wikipedia and subsequently modified substantially.
From what I've seen, nearly every implementation of PL/0 differs from others in some minor way, be it comment syntax or operator selection. Considering its primary role is that of an educational exercise, I've elected not to break this trend and simply select whichever arbitrary conventions suit me best.

In brief, in this version of PL/0:
- comments start with a `#` symbol and terminate at the end of a line
- `==` and `!=` are the equality and inequality operators, respectively
- `?` prints the identifier and value of a variable
- `!` evaluates a following expression, then prints the result

## polski
Prosty interpreter [PL/0](https://en.wikipedia.org/wiki/PL/0), zrobiony w oparciu o [drugą część](https://craftinginterpreters.com/a-tree-walk-interpreter.html) wybitnej książki [Crafting Interpreters](https://craftinginterpreters.com) Roberta Nystroma.

Napisany przeze mnie(Rocha Błachuta) jako projekt ocenowy z przedmiotu Języki Programowania(rok akademicki 2023-2024).

Plik [`pl0.ebnf`](./pl0.ebnf) zawiera opis PL/0's składni w EBNF, który zapożyczyłem z Wikipedii(en), a następnie znacznie zmodyfikowałem.
W moim postrzeganiu prawie wszystkie implementacje PL/0 różnią się od siebie nawzajem drobnymi zmianami, czy to składnia komentarzowa, czy wybór operatorów. Biorąc pod uwagę główną rolę języka(a zwłaszcza tej implementacji) jako ćwiczenie edukacyjne, zadecydowałem zgodnie z tą tendencją wybrać którekolwiek arbitralne konwencje najbardziej mi odpowiadały.

W skrócie, w tej wersji PL/0:
- Komentarze zaczynają się symbolem `#` i kończą na końcu linijki
- `==` i `!=` pełnią role odpowiednio operatora równości i nierówności
- `?` drukuje nazwę oraz wartość zmiennej
- `!` ewaluuje i drukuje wartość wyrażenia
