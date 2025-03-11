import sys
from itertools import product

def calculate_score(ingredients, amounts):
    properties = [0] * (len(ingredients[0]) - 1)  # Excluding calories
    calories = 0
    
    for i, amount in enumerate(amounts):
        for j in range(len(properties)):
            properties[j] += ingredients[i][j] * amount
        calories += ingredients[i][-1] * amount
    
    properties = [max(0, prop) for prop in properties]  # Replace negatives with zero
    score = 1
    for prop in properties:
        score *= prop
    
    return score, calories

def find_best_cookie_score(ingredients, calorie_limit=None):
    num_ingredients = len(ingredients)
    best_score = 0
    
    # Generate all combinations where ingredient amounts sum to 100
    for amounts in product(range(101), repeat=num_ingredients):
        if sum(amounts) == 100:
            score, calories = calculate_score(ingredients, amounts)
            if calorie_limit is None or calories == calorie_limit:
                best_score = max(best_score, score)
    
    return best_score

def main():
    input_file_path = sys.argv[1]
    ingredients = []
    
    with open(input_file_path) as input_file:
        for line in input_file.readlines():
            columns = line.split()
            ingredient = []
            for col in columns:
                try:
                    ingredient.append(int(col.rstrip(",")))
                except ValueError:
                    continue  # Ignore non-numeric values
            ingredients.append(ingredient)
    
    best_score = find_best_cookie_score(ingredients)
    print("Best cookie score:", best_score)
    
    best_score_500_cal = find_best_cookie_score(ingredients, calorie_limit=500)
    print("Best cookie score with 500 calories:", best_score_500_cal)

if __name__ == "__main__":
    main()
