import sys

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

def find_best_cookie_score(ingredients, calorie_limit=None, amounts=None, index=0, remaining=100):
    if index == len(ingredients) - 1:
        amounts.append(remaining)
        score, calories = calculate_score(ingredients, amounts)
        amounts.pop()
        if calorie_limit is None or calories == calorie_limit:
            return score
        return 0
    
    best_score = 0
    for i in range(remaining + 1):
        amounts.append(i)
        best_score = max(best_score, find_best_cookie_score(ingredients, calorie_limit, amounts, index + 1, remaining - i))
        amounts.pop()
    
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
    
    best_score = find_best_cookie_score(ingredients, amounts=[])
    print("Best cookie score:", best_score)
    
    best_score_500_cal = find_best_cookie_score(ingredients, calorie_limit=500, amounts=[])
    print("Best cookie score with 500 calories:", best_score_500_cal)

if __name__ == "__main__":
    main()
