-- defines a factorial function
function fact(n)
    if n == 0 then
        return 1;
    else
        return n * fact(n - 1);
    end
end

print("enter a number:");
a = io.read("*number"); -- read a number
print(fact(a));

io.write(string.format("Hello from %s\n Sin(4)%f", _VERSION, math.sin(4.0)));