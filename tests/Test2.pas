program Test2;
var
   X, A, B : integer;
begin
   A := 5;
   B := 10;
   if (A > B) then
   begin
      X := A;
      A := B;
      B := X   {veja que o ultimo comando de um bloco nao possui o ;}
   end         {alguns alunos preferem implementar com ;}
end.	       

{gere erros sintaticos}