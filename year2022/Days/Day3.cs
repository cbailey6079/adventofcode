namespace year_2022.Days;

public class Day3
{
    private List<string> _input = new();
    private char[] _score = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".ToCharArray();

    public string Part1()
    {
        ParseInput("day3-1");

        var cals = DoStuff();

        return cals.ToString();
    }

    public string Part2()
    {
        ParseInput("day3-1");
        
        var cals = DoStuff2();

        return cals.ToString();
    }

    public string Parttest()
    {
        ParseInput("day3-0");

        var cals = DoStuff2();

        return cals.ToString();
    }

    int DoStuff()
    {
        string[] rucksack = new string[2];
        int total = 0;
        foreach (var row in _input)
        {
            rucksack[0] = row.Substring(0, row.Length/2);
            rucksack[1] = row.Substring(row.Length/2);
            
            // compare to find duplicates;
            var dupe = FindDupe(rucksack);
            // once found we score them
            total += Score(dupe);
        }

        return total;
    }
    
    int DoStuff2()
    {
        string[] rucksack = new string[3];
        int total = 0;
        for (int i = 0; i < _input.Count - 1; i += 3)
        {
            rucksack[0] = _input[i];
            rucksack[1] = _input[i+1];
            rucksack[2] = _input[i+2];
            
            // compare to find duplicates;
            var dupe = FindDupe2(rucksack);
            // once found we score them
            total += Score(dupe);
        }

        return total;
    }

    char FindDupe(string[] rucksack)
    {
        foreach (var item1 in rucksack[0].ToCharArray())
        {
            foreach (var item2 in rucksack[1].ToCharArray())
            {
                if (item1 == item2) return item1;
            }
        }

        char x = '\0';
        
        return x;
    }
    
    char FindDupe2(string[] rucksack)
    {
        foreach (var item1 in rucksack[0].ToCharArray())
        {
            foreach (var item2 in rucksack[1].ToCharArray())
            {
                if (item1 == item2)
                {
                    foreach (var item3 in rucksack[2].ToCharArray())
                    {
                        if (item2 == item3)
                            return item3;
                    }
                }
            }
        }

        char x = '\0';
        
        return x;
    }
    int Score(char dupe)
    {
        return Array.IndexOf(_score, dupe) + 1;
    }

    void ParseInput(string name)
    {
        foreach (string line in File.ReadLines(@"//home/clayton/Code/adventofcode/year2022/Inputs/" + name + ".txt"))
        {
            _input.Add(line);
        }  
    }
}