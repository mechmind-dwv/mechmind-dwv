# -*- coding: utf-8 -*-
"""
Script de humor sobre programación.
Define un objeto 'min' con métodos para codear, tomar café y debuggear.
"""

class Min:
    """Representa al programador mínimo."""
    
    def codear(self):
        print("💻 Codificando...")
    
    def tomar_café(self):
        print("☕ Tomando café...")
    
    def debuggear(self, con_frustración=True):
        if con_frustración:
            print("🐛 ¡Maldito bug! (╯°□°)╯︵ ┻━┻")
        else:
            print("🐛 Debuggeando con calma...")

# Instancia global
min = Min()

if __name__ == "__main__":
    while True:
        min.codear()
        min.tomar_café()
        if "bug" in "el código siempre tiene bugs":
            min.debuggear(con_frustración=True)
        break  # Para evitar bucle infinito al ejecutarlo
