namespace year_2022.Days;

public class Day1
{
    private List<string> _input = new();
    
    public string Part1()
    {
        ParseInput("day1-1");

        var cals = DoStuff();
        
        return cals.Max().ToString();
    }

    public string Part2()
    {
        ParseInput("day1-2");
        
        var cals = DoStuff();
        
        return cals.OrderByDescending(x => x).Take(3).Sum().ToString();
    }

    public string Parttest()
    {
        ParseInput("day1-0");

        var cals = DoStuff();
        
        return cals.OrderByDescending(x => x).Take(3).Sum().ToString();
    }

    List<int> DoStuff()
    {
        List<int> cals = new List<int>();
        int tempTotalCals = 0;
        foreach (var row in _input)
        {
            if (row == "")
            {
                cals.Add(tempTotalCals);
                tempTotalCals = 0;
            }
            else
            {
                tempTotalCals+= Int32.Parse(row);
            }
        }
        cals.Add(tempTotalCals);

        return cals;
    }

    void ParseInput(string name)
    {
        foreach (string line in File.ReadLines(@"/home/clayton/Code/adventofcode/year2022/Inputs/" + name + ".txt"))
        {
            _input.Add(line);
        }  
    }
}