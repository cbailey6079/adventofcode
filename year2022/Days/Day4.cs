namespace year_2022.Days;

public class Day4
{
    private List<string> _input = new();
    
    public string Part1()
    {
        ParseInput("day4-1");

        var overlaps = DoStuff();
        
        return overlaps.ToString();
    }

    public string Part2()
    {
        ParseInput("day4-1");
        
        var overlaps = DoStuff2();
        
        return overlaps.ToString();
    }

    public string Parttest()
    {
        ParseInput("day4-0");

        var overlaps = DoStuff2();
        
        return overlaps.ToString();
    }

    int DoStuff()
    {
        int total = 0;
        bool added;
        foreach (var row in _input)
        {
            added = false;
            var assignments = row.Split(",");
            var elf1 = assignments[0].Split("-");
            var elf2 = assignments[1].Split("-");

            var elf1Lower = Int32.Parse(elf1[0]);
            var elf1Upper = Int32.Parse(elf1[1]);
            var elf2Lower = Int32.Parse(elf2[0]);
            var elf2Upper = Int32.Parse(elf2[1]);
            
            if (elf1Lower <= elf2Lower)
            {
                if (elf1Upper >= elf2Upper)
                {
                    added = true;
                    total++;
                }
            } 
            if (elf1Lower >= elf2Lower && !added)
            {
                if (elf1Upper <= elf2Upper)
                {
                    total++;
                }
            }
        }

        return total;
    }

    int DoStuff2()
    {
        int total = 0;
        bool added;
        foreach (var row in _input)
        {
            added = false;
            var assignments = row.Split(",");
            var elf1 = assignments[0].Split("-");
            var elf2 = assignments[1].Split("-");

            var elf1Lower = Int32.Parse(elf1[0]);
            var elf1Upper = Int32.Parse(elf1[1]);
            var elf2Lower = Int32.Parse(elf2[0]);
            var elf2Upper = Int32.Parse(elf2[1]);
            
            if (elf1Lower <= elf2Lower)
            {
                if (elf1Upper >= elf2Lower)
                {
                    added = true;
                    total++;
                }
            } 
            if (elf1Lower >= elf2Lower && !added)
            {
                if (elf1Lower <= elf2Upper)
                {
                    total++;
                }
            }
        }

        return total;
    }

    void ParseInput(string name)
    {
        foreach (string line in File.ReadLines(@"/home/clayton/Code/adventofcode/year2022/Inputs/" + name + ".txt"))
        {
            _input.Add(line);
        }  
    }
}