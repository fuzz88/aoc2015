def parse_input(file)
    File.readlines(file).map { |line| line.scan(/-?\d+/).map(&:to_i) }
  end
  
  def calculate_score(ingredients, amounts)
    properties = ingredients.transpose.map { |p| p.zip(amounts).map { |x, y| x * y }.sum }
    calories = properties.pop
    [properties.map { |p| [p, 0].max }.reduce(:*) || 0, calories]
  end
  
  def best_score(ingredients, calorie_limit = nil, amounts = [], index = 0, remaining = 100)
    return calculate_score(ingredients, amounts + [remaining]).yield_self { |s, c| calorie_limit.nil? || c == calorie_limit ? s : 0 } if index == ingredients.size - 1
    (0..remaining).map { |i| best_score(ingredients, calorie_limit, amounts + [i], index + 1, remaining - i) }.max
  end
  
  if __FILE__ == $0
    ingredients = parse_input(ARGV[0])
    puts "Best cookie score: #{best_score(ingredients)}"
    puts "Best cookie score with 500 calories: #{best_score(ingredients, 500)}"
  end