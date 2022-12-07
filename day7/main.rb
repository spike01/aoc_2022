class State
  def initialize
    @pwd = "/"
    @dir_stack = []
    @filetree = {}
    @reading_output = false
    @output = []
    @all_files = []
  end

  attr_accessor :pwd, :dir_stack, :filetree, :reading_output, :output, :all_files

  def reading_output?
    reading_output
  end

  def cd(dest)
    if dest == ".."
      @dir_stack.pop
    else
      @dir_stack.push(dest)
    end
    @pwd = dest
  end

  def process_output
    @output.each do |o|
      info, name = o.split(" ").map(&:strip)
      file = AocFile.new(name, info)

      # insert files
      cwd = @dir_stack.reduce(@filetree) do |cwd, popd|
        cwd[popd].size += file.size
        cwd[popd].tree
      end
      all_files.push(file)
      cwd[name] = file
    end
    @output = []
    @reading_output = false
  end
end

class AocFile
  def initialize(name, info)
    @name = name
    if info == "dir"
      @dir = true
      @tree = {}
      @size = 0
    else
      @dir = false
      @size = info.to_i
    end
  end

  attr_accessor :name, :dir, :tree, :size

  def dir?
    @dir
  end
end

f = File.open("./input.txt", "r")

state = State.new
root = AocFile.new("/", "dir")
state.filetree["/"] = root

f.each_line do |l|
  case true
  when l.start_with?("$ cd")
    state.process_output if state.reading_output?
    dest = l[5..].strip
    state.cd(dest);
    next
  when l.start_with?("$ ls")
    state.reading_output = true;
    next
  when state.reading_output? && !l.start_with?("$")
    state.output.push(l.strip);
    next
  end
end

if state.reading_output?
  state.process_output
end

pp state.filetree
pp state.all_files.select { |f| f.size < 100000 && f.dir? }.map(&:size).sum
