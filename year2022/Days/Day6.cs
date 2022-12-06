namespace year_2022.Days;

public class Day6
{
    private List<string> _input = new();
    
    public string Part1()
    {
        ParseInput("day6-1");

        var marker = DoStuff();
        
        return marker.ToString();
    }

    public string Part2()
    {
        ParseInput("day6-1");
        
        var marker = DoStuff(14);
        
        return marker.ToString();
    }

    public string Parttest()
    {
        ParseInput("day6-0");

        var marker = DoStuff(14);
        
        return marker.ToString();
    }

    int DoStuff(int length = 4)
    {
        var dupeFound = false;
        for (int i = 0; i < _input[0].Length - 1 ; i++)
        {
            var check = _input[0].Substring(i, length);

            for (int x = 0; x < check.Length - 1; x++)
            {
                for (int j = x + 1; j < check.Length; j++)
                {
                    if (check[x] == check[j])
                    {
                        dupeFound = true;
                        goto breaker;
                    }
                }
            }
            
            breaker: ;
            if (!dupeFound)
            {
                return i + length;
            }

            dupeFound = false;
        }
        return 0;
    }

    void ParseInput(string name)
    {
        foreach (string line in File.ReadLines(@"/home/clayton/Code/adventofcode/year2022/Inputs/" + name + ".txt"))
        {
            _input.Add(line);
        }  
    }
}