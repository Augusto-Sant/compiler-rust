# Gramatica exemplo

"" = vazio
-nt = non-terminal

program' -> program-nt
program-nt -> fn main ( ) { command-list-nt }
command-list-nt -> command-nt ; command-list-nt
command-list-nt -> ''
command-nt -> assign-nt
command-nt -> print-nt
command-nt -> if-nt
command-nt -> while-nt
assign-nt -> variable-nt = exp-nt
print-nt -> print ( variable ) 
if-nt -> if ( exp-log-nt ) { command-list-nt }
while-nt -> while ( exp-log-nt ) { command-list-nt }
exp-nt -> exp2-nt exp1-nt
exp1-nt -> operator-nt exp2-nt exp1-nt
exp1-nt -> '' 
exp2-nt -> variable-nt 
exp2-nt -> number 
exp-log-nt -> exp-nt exp-log2-nt
exp-log2-nt -> logical-op-nt exp-nt exp-log2-nt 
exp-log2-nt -> '' 
operator-nt -> +
operator-nt -> -
operator-nt -> *
operator-nt -> /
logical-op-nt -> ==
logical-op-nt -> > 
logical-op-nt -> < 
logical-op-nt -> <= 
logical-op-nt -> >= 
variable-nt -> variable


### Lista

0	<PROGRAMA>	<LISTA_COMANDOS>
1	<LISTA_COMANDOS>	<COMANDO><LISTA_COMANDOS>
2	<COMANDO>	"IF|WHILE|FOR|FOREACH|SWITCH|
DO|PRINT|ASSIGN
3	<IF>	if (<EXP>){<LISTA_COMANDOS>}
4	<WHILE>	while(<EXP>){<LISTA_COMANDOS>}
5	<FOR>	for (<ATRIBUICAO> pv <EXP> pv id <INCREMENTO>) {<LISTACOMANDOS>}
6	<FOREACH>	foreach (ATR doispontos id) {<LISTACOMANDOS>}
7	<SWITCH>	switch(EXP) {<CASE> <DEFAULT>}
8	<DO>	do{<LISTA_COMANDOS>} while(EXP)
9	<PRINT>	print(<str>)
10	<CONDICAO>	<EXP> <COMPARA> <EXP>
11	<EXP>	<EXP> <OPERA> <EXP> | <EXP2>
12	<EXP2>	const | id
13	<OPERA>	SOMA|SUBTRACAO|MULTIPLICACAO| ...
14	<BLOCO>	{<LISTA_COMANDOS>}
15	<CONDICAOBLOCO>	(CONDICAO)<BLOCO>
16  <ASSIGN> -> variable = exp
