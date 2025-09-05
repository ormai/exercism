/// <reference path="./global.d.ts" />
// @ts-check

export function cookingStatus(timeLeft) {
  if (timeLeft == undefined) return 'You forgot to set the timer.';
  if (timeLeft === 0) return 'Lasagna is done.';
  return 'Not done, please wait.';
}

export function preparationTime(layers, avgLayerPrepTime) {
  return (avgLayerPrepTime ?? 2) * layers.length;
}

export function quantities(layers) {
  const ingredients = { noodles: 0, sauce: 0 };
  for (const layer of layers) {
    if (layer === 'noodles') {
      ingredients.noodles += 50;
    }
    if (layer === 'sauce') {
      ingredients.sauce += .2;
    }
  }
  return ingredients;
}

export function addSecretIngredient(friendsIngredients, myIngredients) {
  myIngredients.push(friendsIngredients[friendsIngredients.length - 1]);
}

export function scaleRecipe(oldRecipe, portions) {
  const scale = (portions ?? 2) / 2;
  const recipe = Object.assign({}, oldRecipe);
  for (const ingredient in recipe) {
    recipe[ingredient] *= scale;
  }
  return recipe;
}
