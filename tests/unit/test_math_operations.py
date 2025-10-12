from src import add, subtract

class TestMathOperations:
    def test_add_positive(self):
        assert add(2, 3) == 5
    
    def test_add_negative(self):
        assert add(-1, -1) == -2
        
    def test_subtract_positive(self):
        assert subtract(5, 2) == 3
