namespace year_2022.Days;

public class Day11
{
    private List<string> _input = new();
    
    public string Part1()
    {
        ParseInput("day11-1");

        var cals = DoStuff();
        
        return cals;
    }

    public string Part2()
    {
        ParseInput("day11-1");
        
        var cals = DoStuff();
        
        return cals;
    }

    public string Parttest()
    {
        ParseInput("day11-0");

        var cals = DoStuff();
        
        return cals;
    }

    string DoStuff()
    {

        return "";
    }

    void ParseInput(string name)
    {
        foreach (string line in File.ReadLines(@"/home/clayton/Code/adventofcode/year2022/Inputs/" + name + ".txt"))
        {
            _input.Add(line);
        }  
    }
}