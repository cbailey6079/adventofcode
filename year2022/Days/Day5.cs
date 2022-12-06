namespace year_2022.Days;

public class Day5
{
    private List<List<string>> _layout = new();
    private List<string> _inputs = new();
    
    public string Part1()
    {
        ParseInput("day5-1");

        return DoStuff();
    }

    public string Part2()
    {
        ParseInput("day5-1");
        
        return DoStuff(false);
    }

    public string Parttest()
    {
        ParseInput("day5-0");

        return DoStuff(false);
    }

    string DoStuff(bool reverse = true)
    {
        MakeMoves(reverse);
        
        return String.Join("", _layout.Select(stack => stack.Last()));
    }

    void MakeMoves(bool reverse)
    {
        string[] temp;
        int howMany;
        int from;
        int to;
        
        foreach (var input in _inputs)
        {
            temp = input.Split(" ");
            howMany = Int32.Parse(temp[1]);
            from = Int32.Parse(temp[3]) - 1;
            to = Int32.Parse(temp[5]) - 1;

            var tempItems = _layout[from].GetRange(_layout[from].Count - howMany, howMany);
            _layout[from].RemoveRange(_layout[from].Count - howMany, howMany);
            
            if (reverse) tempItems.Reverse();
            _layout[to].AddRange(tempItems);
        }
    }

    void ParseInput(string name)
    {
        var inputs = false;
        var initialized = false;
        foreach (string line in File.ReadLines(@"/home/clayton/Code/adventofcode/year2022/Inputs/" + name + ".txt"))
        {
            if (line == "")
            {
                for (int i = 0; i < _layout.Count - 1; i++)
                {
                    _layout[i].Reverse();
                }
                
                inputs = true;
                continue;
            }
            
            if (!inputs)
            {
                if (line.TrimStart().Substring(0, 1) == "1")
                {
                    continue;
                }

                for (int i = 0; i < line.Length;)
                {
                    if (!initialized)
                    {
                        _layout.Add(new List<string>());
                    }
                    
                    var crate = line.Substring(i + 1, 1);
                    if (crate.Trim() != "")
                    {
                        _layout[i / 4].Add(crate);
                    }

                    i += 4;
                }

                initialized = true;
            } else
            {
                _inputs.Add(line);
            }
        }  
    }
}