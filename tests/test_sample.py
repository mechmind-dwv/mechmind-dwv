from src import add, subtract

def test_addition():
    assert add(2, 3) == 5

def test_subtraction():
    assert subtract(5, 2) == 3
