namespace year_2022.Days;

public class Day10
{
    private List<string> _input = new();
    
    public string Part1()
    {
        ParseInput("day10-1");

        var signalStrength = DoStuff();
        
        return signalStrength;
    }

    public string Part2()
    {
        ParseInput("day10-1");
        
        var signalStrength = renderScreen();
        
        return signalStrength;
    }

    public string Parttest()
    {
        ParseInput("day10-0");

        var signalStrength = renderScreen();
        
        return signalStrength;
    }

    string DoStuff()
    {
        var signalStrength = 0;
        var cycleCalc = new[] {20,60,100,140,180,220};
        var register = 1;
        var cycle = 0;
        var addCycle = 1;
        
        for (int i = 0; i < _input.Count;)
        {
            var command = _input[i].Split(" ");
            cycle++;
            
            if (cycleCalc.Contains(cycle))
            {
                signalStrength += cycle * register;
            }
            
            if (command[0] == "addx")
            {
                if (addCycle == 2)
                {
                    register += Int32.Parse(command[1]);
                    addCycle = 1;
                    i++;
                }
                else
                {
                    addCycle++;
                }
            }
            else
            {
                i++;
            }
        }
        
        return signalStrength.ToString();
    }
    
    string renderScreen()
    {
        var register = 1;
        var cycle = 0;
        var addCycle = 1;
        var spritePos = new[] {0, 1, 2};
        var crt = new string[6,40];
        
        for (int i = 0; i < _input.Count;)
        {
            var command = _input[i].Split(" ");
            cycle++;
            
            if (spritePos.Contains((cycle - 1) % 40))
            {
                crt[(cycle - 1) / 40, (cycle - 1) % 40] = "#";
            }
            else
            {
                crt[(cycle - 1) / 40, (cycle - 1) % 40] = ".";
            }
            
            if (command[0] == "addx")
            {
                if (addCycle == 2)
                {
                    register += Int32.Parse(command[1]);
                    addCycle = 1;
                    i++;
                }
                else
                {
                    addCycle++;
                }
            }
            else
            {
                i++;
            }
            
            // update sprite position
            spritePos = new[] { register - 1, register, register + 1 };
        }

        return print(crt);
    }

    void ParseInput(string name)
    {
        foreach (string line in File.ReadLines(@"/home/clayton/Code/adventofcode/year2022/Inputs/" + name + ".txt"))
        {
            _input.Add(line);
        }  
    }

    string print(string[,] crt)
    {
        var print = "";
        
        for (int i = 0; i < crt.GetLength(0); i++)
        {
            for (int j = 0; j < crt.GetLength(1); j++)
            {
                print += crt[i, j];
            }

            print += "\n";
        }

        return print;
    }
}