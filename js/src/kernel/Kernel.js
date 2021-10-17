export class Kernel
{
    static errors = [
        "Can't move cursor to the left on line $l, character $c, because cursor is on first position!",
        "Unexpected loop-ending character at line $l, character $c!",
        "Unexpected number addition on line $l, character $c, position $p can't be bigger than 255!",
        "Unexpected number subtraction on line $l, character $c, position $p can't be smaller than -255!",
        "Maximum of allowed cycles (255) was reached on line $l, character $c!"
    ]

    constructor(code)
    {
        this.code = code;
        this.tokens = code.split("");
        this.last_token = 0;
        this.last_line = 0;
        this.position = 0;
        this.stack = [0];
        this.isParsingLoop = false;
        this.loop_string = "";
        this.loop_cycle = 0;
        this.result =[];
    }

    parse()
    {
        for (var token in this.tokens)
            if (this.parseToken(this.tokens[token]) == 1)
                return this.result;
        
        return this.result;
    }

    parseToken(token)
    {
        if (token == "]")
        {
            if (!this.isParsingLoop)
            {
                this.last_token++;
                return this.error(1);
            }

            if (!this.loop_string.includes("["))
            {      
                this.isParsingLoop = false;
                this.isLooping = true;
                while (this.stack[this.position] != 0)
                {
                    this.loop_cycle++;
                    if (this.loop_cycle == 256)
                    {
                        this.last_token++;
                        this.error(4);
                        return this.result;
                    }
                    for (var loop_token in this.loop_string.split(""))
                        if (this.parseToken(this.loop_string[loop_token]) == 1)
                            return 1;
                    this.last_token -= this.loop_string.length;
                }
                this.loop_cycle = 0;
                this.isLooping = false;
                this.loop_string = "";
            }
        }

        if (this.isParsingLoop)
            return this.loop_string+= token;
        
        this.last_token++;
        
        switch(token)
        {
            case ">":
                this.position++;
                if (this.stack.length < this.position+1)
                    this.stack.push(0);
                break;
            case "<":
                if (this.position == 0)
                    return this.error(0);
                this.position--;
                break;
            case "+":
                if (this.stack[this.position] == 255)
                    return this.error(2);
                this.stack[this.position]++;
                break;
            case "-":
                if (this.stack[this.position] == -255)
                    return this.error(3);
                this.stack[this.position]--;
                break;
            case ".":
                this.out("char", String.fromCharCode(this.stack[this.position]));
                break;
            case "_":
                this.out("int", this.stack[this.position]);
                break;
            case "[":
                this.isParsingLoop = true;
                break;
            case "\n":
                this.last_line++;
                this.last_token = 0;
                break;
        }
    }

    error(code)
    {
        var message = Kernel.errors[code];
        message = message.replace("$c", this.last_token);
        message = message.replace("$p", this.position);
        message = message.replace("$l", this.last_line+1);
        this.out("error", "Error " + code + ": " + message); 
        return 1;
    }
    
    out(type, content)
    {
        this.result.push({type: type, content: content});
    }

}
