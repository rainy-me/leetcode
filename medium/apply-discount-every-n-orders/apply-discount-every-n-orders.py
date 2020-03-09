class Cashier:

    def __init__(self, n: int, discount: int, products: List[int], prices: List[int]):
        self.n = n
        self.discount = discount
        self.count = 0
        self.products = dict(zip(products, prices))

    def getBill(self, product: List[int], amount: List[int]) -> float:
        self.count += 1
        bill = sum([self.products[p] * a for [p, a] in zip(product, amount)])
        if self.count == self.n:
            self.count = 0
            bill -= (self.discount * bill) / 100
        return bill
