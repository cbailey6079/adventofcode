namespace year_2022.Days;

public class Day2
{
    private List<string> _input = new();
    
    public string Part1()
    {
        ParseInput("day2-1");

        var cals = DoStuff();

        return cals.ToString();
    }

    public string Part2()
    {
        ParseInput("day2-1");
        
        var cals = DoStuff2();

        return cals.ToString();
    }

    public string Parttest()
    {
        ParseInput("day2-0");

        var cals = DoStuff();

        return cals.ToString();
    }

    int DoStuff()
    {
        int total = 0;
        foreach (var row in _input)
        {
            var round = row.Split(" ");
            total += PlayedScore(round[1]) + RoundScore(round[0], round[1]);
        }

        return total;
    }
    
    int DoStuff2()
    {
        int total = 0;
        foreach (var row in _input)
        {
            var round = row.Split(" ");

            var yours = GetYours(round[0], round[1]);
            total += PlayedScore(yours) + RoundScore(round[0], yours);
        }

        return total;
    }

    int PlayedScore(string yours)
    {
        switch (yours)
        {
            case "Y":
                return 2;
            case "X":
                return 1;
            case "Z":
                return 3;
            default:
                return 0;
        }
    }

    int RoundScore(string opponent, string yours)
    {
        if ((opponent == "A" && yours == "X") || 
            (opponent == "B" && yours == "Y") || 
            (opponent == "C" && yours == "Z"))
        {
            return 3;
        } else if (opponent == "A" && yours == "Y" ||
                   opponent == "B" && yours == "Z" ||
                   opponent == "C" && yours == "X")
        {
            return 6;
        }
        return 0;
    }
    
    string GetYours(string opponent, string yours)
    {
        switch (opponent)
        {
            case "A":
                if (yours == "Z")
                    return "Y";
                if (yours == "Y")
                    return "X";
                if (yours == "X")
                    return "Z";
                break;
            case "B":
                if (yours == "Z")
                    return "Z";
                if (yours == "Y")
                    return "Y";
                if (yours == "X")
                    return "X";
                break;
            case "C":
                if (yours == "Z")
                    return "X";
                if (yours == "Y")
                    return "Z";
                if (yours == "X")
                    return "Y";
                break;
        }
        return "";
    }

    void ParseInput(string name)
    {
        foreach (string line in File.ReadLines(@"//home/clayton/Code/adventofcode/year2022/Inputs/" + name + ".txt"))
        {
            _input.Add(line);
        }  
    }
}