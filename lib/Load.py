import csv
import sqlite3

def load(dataset="SPX.csv"):
    # Connect to SQLite database
    conn = sqlite3.connect('Index.db')
    cursor = conn.cursor()

    # Create a table named 'SPX'
    cursor.execute('''
    CREATE TABLE IF NOT EXISTS SPX (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        date DOUBLE,
        open DOUBLE,
        high DOUBLE,
        low DOUBLE,
        close DOUBLE,
        adj_close DOUBLE,
        volume DOUBLE,
    )
    ''')

    # Read the CSV file and insert data into the SQLite database
    with open(dataset, 'r') as csv_file:
        csv_reader = csv.DictReader(csv_file)
        for row in csv_reader:
            cursor.execute('''
            INSERT INTO SPX (date, open, high, low, close, adj_close, volume) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ''', (row["Date"], row["Open"], row["High"], row["Low"], row["Close"], row["Adj Close"], row["Volume"]))

    # Commit changes and close the SQLite connection
    conn.commit()
    conn.close()
