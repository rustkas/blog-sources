-module(chapter03).
-export([
    greet/2, first/1, 
    second/1, 
    same/2, 
    bmi_tell/1, 
    lucky_number/1, 
    lucky_atom/1,
    safe_division/2,
    if_bmi_tell/1,
    assessment_of_temp/1,
    length/1
]).
-author("Haru Atari").

greet(male, Name) ->
    io:format("Hello, Mr. ~s!~n", [Name]);
greet(female, Name) ->
    io:format("Hello, Mrs. ~s!~n", [Name]);
greet(_, Name) ->
    io:format("Hello, ~s!~n", [Name]).

first([X|_])->
    X.

second([_,X|_]) ->
    X.

same(X,X) -> 
    true;
same(_,_) ->
    false.

bmi_tell(Bmi) when Bmi =< 18.5 ->
    "You're underweight.~n";
bmi_tell(Bmi) when Bmi =< 25 ->
   "You're supposedly normal.~n";
bmi_tell(Bmi) when Bmi =< 30 ->
   "You're fat.~n";
bmi_tell(_) ->
    "You're very fat.~n".

lucky_number(X) when 10 < X, X < 20 ->
    true;
lucky_number(_) ->
    false.

lucky_atom(X) when X == atom1; X == atom2 ->
    true;
lucky_atom(_) ->
    false.

safe_division(X, Y) when is_number(X), is_number(Y), Y /= 0 ->
    X / Y;
safe_division(_, _) ->
    false.

if_bmi_tell(Bmi) ->
    if Bmi =< 18.5 -> "You're underweight.~n";
       Bmi =< 25   -> "You're supposedly normal.~n";
       Bmi =< 30   -> "You're fat.~n";
       true        -> "You're very fat.~n"
    end.

assessment_of_temp(Temp) ->
    case Temp of
        {X, celsius} when 20 =< X, X =< 45 ->
            'favorable';
        {X, kelvin} when 293 =< X, X =< 318 ->
            'scientifically favorable';
        {X, fahrenheit} when 68 =< X, X =< 113 ->
            'favorable in the US';
        _ ->
            'not the best tempperature'
  end.

length(L) -> length(L, 0).
length([], Total) -> Total;
length([_|T], Total) -> length(T, Total + 1).
