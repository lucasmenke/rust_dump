# rust dump

A collection of sample rust projects.

<ins>Basics</ins>
1. DB in Projekt einfügen 
2. Tab Control 
  - Dock -> Fill
3. Split Container 
4. Datenquellen per Dropdown "Details" auswählen (z.B. Kunde) 
5. Per Drag and Drop in Split ziehen 
6. Navigator oben anklicken 
  - Dock -> None 
  - In Split ziehen 
  - Dock -> Top 
7. Datenquellen "DataView" hinzufügen die mit "Details" aus 4 korrelliert (Kunde > Bestellungen) 
  - Datenquelle aus 3. aufklappen 
  - Tabelle per Drag and Drop hinzufügen 
  - Dock -> Fill 

 

<ins>Combo Box</ins>

Tabelle 

1. Auf Tabelle klicken (z.B. Bestelldetails) 
2. Columns anklicken in Eigenschaften
3. Spalte auswählen 
  - Column Type -> Combo Box 
  - Data Source -> weitere Datenquellen (z.B. Artikel) 
  - Display Member -> was angezeigt wird -> Spalte auswählen (z.B. Artikelnamen) 
  - Value Member -> worauf das angezeigt bassiert 

 

Details 
1. Text Feld löschen  
2. Combo Box reinziehen
3. Eigenschaften anpassen
  - Data Source -> weitere Datenquellen (z.B. Artikel) 
  - Display Member -> was angezeigt wird -> Spalte auswählen (z.B. Artikelnamen) 
  - Value Member -> worauf das angezeigt bassiert 
  - DataBindings -> Selected Value (z.B. Bestellungen -> Personal Nr) 

<ins>Expressions</ins>

1. NordwindDataSet.xsd öffnen
2. neues Feld hinzufügen
3. Expression -> Parent(ArtikelBestelldetails).Artikelnamen

 
``` csharp
  private void button1_Click(object sender, EventArgs e) 
  { 
      string priceChange = textBox1.Text.Trim(); 

      try 
      { 
          var change = Convert.ToDecimal(priceChange); 
          var articles = nordwindDataSet1.Artikel.Where(x => x.KategorieNr == (int)comboBox1.SelectedValue).ToList(); 
          articles.ForEach(x => x.Einzelpreis = x.Einzelpreis * ((100 + change) / 100)); 
          artikelTableAdapter1.Update(nordwindDataSet1.Artikel); 
      } 
      catch (Exception ex) 
      { 
          MessageBox.Show($"Fehler: {ex}"); 
      } 
  } 
 
  private void button1_Click(object sender, EventArgs e) 
  { 
      foreach (var artikel in nordwindDataSet1.Artikel) 
      { 
          if (artikel.Einzelpreis > 10) 
          { 
              listBox1.Items.Add(artikel.Artikelname); 
          } 
      } 
  }  

  private void Form1_Load(object sender, EventArgs e) 
  { 
      var bestellungen = nordwindDataSet.Bestellungen.Where(b => b.KundenCode == kundenCodeTextBox.Text); 
      bestellungen = bestellungen.OrderBy(b => b.Bestelldatum); 
      textBox1.Text = bestellungen.First().Bestelldatum.ToString(); 
  }
```
