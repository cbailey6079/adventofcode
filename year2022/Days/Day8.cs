namespace year_2022.Days;

public class Day8
{
    private int[,] _input;
    
    public string Part1()
    {
        ParseInput("day8-1");

        var visibleTrees = DoStuff();
        
        return visibleTrees.ToString();
    }

    public string Part2()
    {
        ParseInput("day8-1");
        
        var scenicScore = DoStuff2();
        
        return scenicScore.ToString();
    }

    public string Parttest()
    {
        ParseInput("day8-0");

        var scenicScore = DoStuff2();
        
        return scenicScore.ToString();
    }

    int DoStuff()
    {
        int visibleTrees = 0;
        for (int i = 1; i < _input.GetLength(0) - 1; i++)
        {
            for (int j = 1; j < _input.GetLength(1) - 1; j++)
            {
                var current = _input[i, j]; 
                if (CheckUp(current, i,j) ||
                    CheckDown(current, i,j) ||
                    CheckLeft(current, i,j) ||
                    CheckRight(current, i,j))
                {
                    visibleTrees++;
                }
            }
        }
        
        return visibleTrees + (_input.GetLength(0) * 4) - 4;
    }
    
    int DoStuff2()
    {
        int scenicScore = 0;
        for (int i = 1; i < _input.GetLength(0) - 1; i++)
        {
            for (int j = 1; j < _input.GetLength(1) - 1; j++)
            {
                var current = _input[i, j];
                var tempScore = CheckUp2(current, i, j) * CheckDown2(current, i, j) * CheckLeft2(current, i, j) * CheckRight2(current, i,j);
                
                if (tempScore > scenicScore) scenicScore = tempScore;
            }
        }

        return scenicScore;
    }

    bool CheckLeft(int tree, int x, int y)
    {
        for (int i = 1; i < y + 1; i++)
        {
            if (_input[x, y - i] >= tree) return false;
        }

        return true;
    }
    
    bool CheckRight(int tree, int x, int y)
    {
        for (int i = 1; y + i < _input.GetLength(1); i++)
        {
            if (_input[x, y + i] >= tree) return false;
        }

        return true;
    }
    
    bool CheckUp(int tree, int x, int y)
    {
        for (int i = 1; i < x + 1; i++)
        {
            if (_input[x - i, y] >= tree) return false;
        }

        return true;
    }
    
    bool CheckDown(int tree, int x, int y)
    {
        for (int i = 1; x + i < _input.GetLength(0); i++)
        {
            if (_input[x + i, y] >= tree) return false;
        }

        return true;
    }
    
    int CheckLeft2(int tree, int x, int y)
    {
        var range = 0;
        for (int i = 1; i < y + 1; i++)
        {
            if (_input[x, y - i] >= tree) return range + 1;

            range++;
        }

        return range;
    }
    
    int CheckRight2(int tree, int x, int y)
    {
        var range = 0;
        for (int i = 1; y + i < _input.GetLength(1); i++)
        {
            if (_input[x, y + i] >= tree) return range + 1;
            range++;
        }

        return range;
    }
    
    int CheckUp2(int tree, int x, int y)
    {
        var range = 0;
        for (int i = 1; i < x + 1; i++)
        {
            if (_input[x - i, y] >= tree) return range + 1;
            range++;
        }

        return range;
    }
    
    int CheckDown2(int tree, int x, int y)
    {
        var range = 0;
        for (int i = 1; x + i < _input.GetLength(0); i++)
        {
            if (_input[x + i, y] >= tree) return range + 1;
            range++;
        }

        return range;
    }

    void ParseInput(string name)
    {
        bool assigned = false;
        int y = 0;
        foreach (string line in File.ReadLines(@"/home/clayton/Code/adventofcode/year2022/Inputs/" + name + ".txt"))
        {
            if (!assigned)
            {
                _input = new int[line.Length, line.Length];
                assigned = true;
            }

            var x = 0;
            foreach (var tree in line)
            {
                _input[y, x] = Int32.Parse(tree.ToString());
                x++;
            }
            
            y++;
        }  
    }
}