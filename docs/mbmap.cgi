<html>
  <head>
    <title>Modbus Mapping - Guntamatic</title>
    <meta charset="iso-8859-1" />
    <style>
      table {
        border-collapse: collapse;
        text-align: center;
      }
      table,
      td,
      th {
        border: 1px solid black;
        padding: 2px;
      }
      th {
        vertical-align: top;
      }
    </style>
  </head>
  <body cz-shortcut-listen="true">
    <h1>Modbus Mapping</h1>
    <table>
      <thead>
        <tr>
          <th>Id</th>
          <th>Register</th>
          <th>Adresse</th>
          <th>Typ</th>
          <th>Einheit</th>
          <th>Größe<br />(Byte)</th>
          <th>Name</th>
          <th>aktueller Wert</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>0</td>
          <td>0x4001</td>
          <td>0x4000</td>
          <td>string</td>
          <td></td>
          <td>4</td>
          <td>Betrieb</td>
          <td><code>0x52454745</code>REGE</td>
        </tr>
        <tr>
          <td>1</td>
          <td>0x4003</td>
          <td>0x4002</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Aussentemperatur</td>
          <td><code>0x3e9e8c10</code>0.31</td>
        </tr>
        <tr>
          <td>3</td>
          <td>0x4007</td>
          <td>0x4006</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Kesseltemperatur</td>
          <td><code>0x4243b5ca</code>48.93</td>
        </tr>
        <tr>
          <td>5</td>
          <td>0x400b</td>
          <td>0x400a</td>
          <td>float</td>
          <td>%</td>
          <td>4</td>
          <td>Leistung</td>
          <td><code>0x42be07d1</code>95.02</td>
        </tr>
        <tr>
          <td>8</td>
          <td>0x4011</td>
          <td>0x4010</td>
          <td>float</td>
          <td>%</td>
          <td>4</td>
          <td>CO2 Gehalt</td>
          <td><code>0x4131c67d</code>11.11</td>
        </tr>
        <tr>
          <td>10</td>
          <td>0x4015</td>
          <td>0x4014</td>
          <td>int</td>
          <td></td>
          <td>4</td>
          <td>Betriebscode</td>
          <td><code>0x00000000</code>0</td>
        </tr>
        <tr>
          <td>17</td>
          <td>0x4023</td>
          <td>0x4022</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Puffer oben</td>
          <td><code>0x4202feb0</code>32.75</td>
        </tr>
        <tr>
          <td>19</td>
          <td>0x4027</td>
          <td>0x4026</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Puffer unten</td>
          <td><code>0x41d314ff</code>26.39</td>
        </tr>
        <tr>
          <td>20</td>
          <td>0x4029</td>
          <td>0x4028</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>Pumpe HP0</td>
          <td><code>0x00000000</code>0</td>
        </tr>
        <tr>
          <td>21</td>
          <td>0x402b</td>
          <td>0x402a</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Warmwasser 0</td>
          <td><code>0x4257df8f</code>53.97</td>
        </tr>
        <tr>
          <td>22</td>
          <td>0x402d</td>
          <td>0x402c</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>P Warmwasser 0</td>
          <td><code>0x00000000</code>0</td>
        </tr>
        <tr>
          <td>23</td>
          <td>0x402f</td>
          <td>0x402e</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Warmwasser 1</td>
          <td><code>0xc1a00000</code>-20.00</td>
        </tr>
        <tr>
          <td>24</td>
          <td>0x4031</td>
          <td>0x4030</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>P Warmwasser 1</td>
          <td><code>0x00000000</code>0</td>
        </tr>
        <tr>
          <td>25</td>
          <td>0x4033</td>
          <td>0x4032</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Warmwasser 2</td>
          <td><code>0xc1a00000</code>-20.00</td>
        </tr>
        <tr>
          <td>26</td>
          <td>0x4035</td>
          <td>0x4034</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>P Warmwasser 2</td>
          <td><code>0x00000000</code>0</td>
        </tr>
        <tr>
          <td>27</td>
          <td>0x4037</td>
          <td>0x4036</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Raumtemp. HK 0</td>
          <td><code>0x42700000</code>60.00</td>
        </tr>
        <tr>
          <td>28</td>
          <td>0x4039</td>
          <td>0x4038</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>Heizkreis 0</td>
          <td><code>0x00000000</code>0</td>
        </tr>
        <tr>
          <td>29</td>
          <td>0x403b</td>
          <td>0x403a</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Raumtemp. HK 1</td>
          <td><code>0x42700000</code>60.00</td>
        </tr>
        <tr>
          <td>31</td>
          <td>0x403f</td>
          <td>0x403e</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Vorlauf Ist 1</td>
          <td><code>0x4232b700</code>44.68</td>
        </tr>
        <tr>
          <td>33</td>
          <td>0x4043</td>
          <td>0x4042</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>Heizkreis 1</td>
          <td><code>0x00000001</code>1</td>
        </tr>
        <tr>
          <td>34</td>
          <td>0x4045</td>
          <td>0x4044</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Raumtemp. HK 2</td>
          <td><code>0x42700000</code>60.00</td>
        </tr>
        <tr>
          <td>36</td>
          <td>0x4049</td>
          <td>0x4048</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Vorlauf Ist 2</td>
          <td><code>0x4202bdda</code>32.69</td>
        </tr>
        <tr>
          <td>38</td>
          <td>0x404d</td>
          <td>0x404c</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>Heizkreis 2</td>
          <td><code>0x00000001</code>1</td>
        </tr>
        <tr>
          <td>39</td>
          <td>0x404f</td>
          <td>0x404e</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Raumtemp. HK 3</td>
          <td><code>0xc1100000</code>-9.00</td>
        </tr>
        <tr>
          <td>40</td>
          <td>0x4051</td>
          <td>0x4050</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>Heizkreis 3</td>
          <td><code>0x00000000</code>0</td>
        </tr>
        <tr>
          <td>41</td>
          <td>0x4053</td>
          <td>0x4052</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Raumtemp. HK 4</td>
          <td><code>0xc1100000</code>-9.00</td>
        </tr>
        <tr>
          <td>43</td>
          <td>0x4057</td>
          <td>0x4056</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Vorlauf Ist 4</td>
          <td><code>0xc1a00000</code>-20.00</td>
        </tr>
        <tr>
          <td>45</td>
          <td>0x405b</td>
          <td>0x405a</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>Heizkreis 4</td>
          <td><code>0x00000000</code>0</td>
        </tr>
        <tr>
          <td>46</td>
          <td>0x405d</td>
          <td>0x405c</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Raumtemp. HK 5</td>
          <td><code>0xc1100000</code>-9.00</td>
        </tr>
        <tr>
          <td>48</td>
          <td>0x4061</td>
          <td>0x4060</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Vorlauf Ist 5</td>
          <td><code>0x42400000</code>48.00</td>
        </tr>
        <tr>
          <td>50</td>
          <td>0x4065</td>
          <td>0x4064</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>Heizkreis 5</td>
          <td><code>0x00000000</code>0</td>
        </tr>
        <tr>
          <td>51</td>
          <td>0x4067</td>
          <td>0x4066</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Raumtemp. HK 6</td>
          <td><code>0xc1100000</code>-9.00</td>
        </tr>
        <tr>
          <td>52</td>
          <td>0x4069</td>
          <td>0x4068</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>Heizkreis 6</td>
          <td><code>0x00000000</code>0</td>
        </tr>
        <tr>
          <td>53</td>
          <td>0x406b</td>
          <td>0x406a</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Raumtemp. HK 7</td>
          <td><code>0xc1100000</code>-9.00</td>
        </tr>
        <tr>
          <td>55</td>
          <td>0x406f</td>
          <td>0x406e</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Vorlauf Ist 7</td>
          <td><code>0xc1a00000</code>-20.00</td>
        </tr>
        <tr>
          <td>57</td>
          <td>0x4073</td>
          <td>0x4072</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>Heizkreis 7</td>
          <td><code>0x00000000</code>0</td>
        </tr>
        <tr>
          <td>58</td>
          <td>0x4075</td>
          <td>0x4074</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Raumtemp. HK 8</td>
          <td><code>0xc1100000</code>-9.00</td>
        </tr>
        <tr>
          <td>60</td>
          <td>0x4079</td>
          <td>0x4078</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Vorlauf Ist 8</td>
          <td><code>0x42400000</code>48.00</td>
        </tr>
        <tr>
          <td>62</td>
          <td>0x407d</td>
          <td>0x407c</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>Heizkreis 8</td>
          <td><code>0x00000000</code>0</td>
        </tr>
        <tr>
          <td>68</td>
          <td>0x4089</td>
          <td>0x4088</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>Kesselfreigabe</td>
          <td><code>0x00000001</code>1</td>
        </tr>
        <tr>
          <td>69</td>
          <td>0x408b</td>
          <td>0x408a</td>
          <td>string</td>
          <td></td>
          <td>4</td>
          <td>Programm</td>
          <td><code>0x4e4f524d</code>NORM</td>
        </tr>
        <tr>
          <td>70</td>
          <td>0x408d</td>
          <td>0x408c</td>
          <td>string</td>
          <td></td>
          <td>4</td>
          <td>Progamm HK0</td>
          <td><code>0x41555300</code>AUS</td>
        </tr>
        <tr>
          <td>71</td>
          <td>0x408f</td>
          <td>0x408e</td>
          <td>string</td>
          <td></td>
          <td>4</td>
          <td>Progamm HK1</td>
          <td><code>0x4845495a</code>HEIZ</td>
        </tr>
        <tr>
          <td>72</td>
          <td>0x4091</td>
          <td>0x4090</td>
          <td>string</td>
          <td></td>
          <td>4</td>
          <td>Progamm HK2</td>
          <td><code>0x4845495a</code>HEIZ</td>
        </tr>
        <tr>
          <td>73</td>
          <td>0x4093</td>
          <td>0x4092</td>
          <td>string</td>
          <td></td>
          <td>4</td>
          <td>Progamm HK3</td>
          <td><code>0x41555300</code>AUS</td>
        </tr>
        <tr>
          <td>74</td>
          <td>0x4095</td>
          <td>0x4094</td>
          <td>string</td>
          <td></td>
          <td>4</td>
          <td>Progamm HK4</td>
          <td><code>0x41555300</code>AUS</td>
        </tr>
        <tr>
          <td>75</td>
          <td>0x4097</td>
          <td>0x4096</td>
          <td>string</td>
          <td></td>
          <td>4</td>
          <td>Progamm HK5</td>
          <td><code>0x41555300</code>AUS</td>
        </tr>
        <tr>
          <td>76</td>
          <td>0x4099</td>
          <td>0x4098</td>
          <td>string</td>
          <td></td>
          <td>4</td>
          <td>Progamm HK6</td>
          <td><code>0x41555300</code>AUS</td>
        </tr>
        <tr>
          <td>77</td>
          <td>0x409b</td>
          <td>0x409a</td>
          <td>string</td>
          <td></td>
          <td>4</td>
          <td>Progamm HK7</td>
          <td><code>0x41555300</code>AUS</td>
        </tr>
        <tr>
          <td>78</td>
          <td>0x409d</td>
          <td>0x409c</td>
          <td>string</td>
          <td></td>
          <td>4</td>
          <td>Progamm HK8</td>
          <td><code>0x41555300</code>AUS</td>
        </tr>
        <tr>
          <td>81</td>
          <td>0x40a3</td>
          <td>0x40a2</td>
          <td>int</td>
          <td></td>
          <td>4</td>
          <td>Serial</td>
          <td><code>0x00231904</code>2300164</td>
        </tr>
        <tr>
          <td>83</td>
          <td>0x40a7</td>
          <td>0x40a6</td>
          <td>int</td>
          <td>h</td>
          <td>4</td>
          <td>Betriebszeit</td>
          <td><code>0x00002982</code>10626</td>
        </tr>
        <tr>
          <td>84</td>
          <td>0x40a9</td>
          <td>0x40a8</td>
          <td>int</td>
          <td>d</td>
          <td>4</td>
          <td>Servicezeit</td>
          <td><code>0x0000053a</code>1338</td>
        </tr>
        <tr>
          <td>85</td>
          <td>0x40ab</td>
          <td>0x40aa</td>
          <td>int</td>
          <td>h</td>
          <td>4</td>
          <td>Asche leeren in</td>
          <td><code>0x0000003d</code>61</td>
        </tr>
        <tr>
          <td>86</td>
          <td>0x40ad</td>
          <td>0x40ac</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Vorlauf Ist 0</td>
          <td><code>0x42400000</code>48.00</td>
        </tr>
        <tr>
          <td>87</td>
          <td>0x40af</td>
          <td>0x40ae</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Vorlauf Ist 3</td>
          <td><code>0x42400000</code>48.00</td>
        </tr>
        <tr>
          <td>88</td>
          <td>0x40b1</td>
          <td>0x40b0</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Vorlauf Ist 6</td>
          <td><code>0x42400000</code>48.00</td>
        </tr>
        <tr>
          <td>89</td>
          <td>0x40b3</td>
          <td>0x40b2</td>
          <td>float</td>
          <td>m3</td>
          <td>4</td>
          <td>Brennstoffzähler</td>
          <td><code>0x3f261c85</code>0.65</td>
        </tr>
        <tr>
          <td>90</td>
          <td>0x40b5</td>
          <td>0x40b4</td>
          <td>int</td>
          <td>%</td>
          <td>4</td>
          <td>Pufferladung</td>
          <td><code>0x00000000</code>0</td>
        </tr>
        <tr>
          <td>91</td>
          <td>0x40b7</td>
          <td>0x40b6</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Puffer oben 0</td>
          <td><code>0xc1a00000</code>-20.00</td>
        </tr>
        <tr>
          <td>92</td>
          <td>0x40b9</td>
          <td>0x40b8</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Puffer unten 0</td>
          <td><code>0xc1a00000</code>-20.00</td>
        </tr>
        <tr>
          <td>93</td>
          <td>0x40bb</td>
          <td>0x40ba</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Puffer oben 1</td>
          <td><code>0xc1a00000</code>-20.00</td>
        </tr>
        <tr>
          <td>94</td>
          <td>0x40bd</td>
          <td>0x40bc</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Puffer unten 1</td>
          <td><code>0xc1a00000</code>-20.00</td>
        </tr>
        <tr>
          <td>95</td>
          <td>0x40bf</td>
          <td>0x40be</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Puffer oben 2</td>
          <td><code>0xc1a00000</code>-20.00</td>
        </tr>
        <tr>
          <td>96</td>
          <td>0x40c1</td>
          <td>0x40c0</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Puffer unten 2</td>
          <td><code>0xc1a00000</code>-20.00</td>
        </tr>
        <tr>
          <td>97</td>
          <td>0x40c3</td>
          <td>0x40c2</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>P Zusatzwarmw. 0</td>
          <td><code>0x00000000</code>0</td>
        </tr>
        <tr>
          <td>98</td>
          <td>0x40c5</td>
          <td>0x40c4</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>P Zusatzwarmw. 1</td>
          <td><code>0x00000000</code>0</td>
        </tr>
        <tr>
          <td>99</td>
          <td>0x40c7</td>
          <td>0x40c6</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>P Zusatzwarmw. 2</td>
          <td><code>0x00000000</code>0</td>
        </tr>
        <tr>
          <td>100</td>
          <td>0x40c9</td>
          <td>0x40c8</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>Fernpumpe 0</td>
          <td><code>0x00000000</code>0</td>
        </tr>
        <tr>
          <td>101</td>
          <td>0x40cb</td>
          <td>0x40ca</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>Fernpumpe 1</td>
          <td><code>0x00000000</code>0</td>
        </tr>
        <tr>
          <td>102</td>
          <td>0x40cd</td>
          <td>0x40cc</td>
          <td>bool</td>
          <td></td>
          <td>4</td>
          <td>Fernpumpe 2</td>
          <td><code>0x00000000</code>0</td>
        </tr>
        <tr>
          <td>108</td>
          <td>0x40d9</td>
          <td>0x40d8</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Puffer T5</td>
          <td><code>0xc2c80000</code>-100.00</td>
        </tr>
        <tr>
          <td>109</td>
          <td>0x40db</td>
          <td>0x40da</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Puffer T6</td>
          <td><code>0xc2c80000</code>-100.00</td>
        </tr>
        <tr>
          <td>110</td>
          <td>0x40dd</td>
          <td>0x40dc</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Puffer T7</td>
          <td><code>0xc1a00000</code>-20.00</td>
        </tr>
        <tr>
          <td>111</td>
          <td>0x40df</td>
          <td>0x40de</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Zusatzwarmw. 0</td>
          <td><code>0x42f00000</code>120.00</td>
        </tr>
        <tr>
          <td>112</td>
          <td>0x40e1</td>
          <td>0x40e0</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Zusatzwarmw. 1</td>
          <td><code>0xc1a00000</code>-20.00</td>
        </tr>
        <tr>
          <td>113</td>
          <td>0x40e3</td>
          <td>0x40e2</td>
          <td>float</td>
          <td>°C</td>
          <td>4</td>
          <td>Zusatzwarmw. 2</td>
          <td><code>0xc1a00000</code>-20.00</td>
        </tr>
      </tbody>
    </table>
    <h2>Erweiterte Texte</h2>
    <table>
      <thead>
        <tr>
          <th>Id</th>
          <th>Register</th>
          <th>Adresse</th>
          <th>Größe<br />(Byte)</th>
          <th>Name</th>
          <th>aktueller Wert</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>0</td>
          <td>0x5001</td>
          <td>0x5000</td>
          <td>64</td>
          <td>Betrieb</td>
          <td>REGELUNG</td>
        </tr>
        <tr>
          <td>69</td>
          <td>0x50e1</td>
          <td>0x50e0</td>
          <td>64</td>
          <td>Programm</td>
          <td>NORMAL</td>
        </tr>
        <tr>
          <td>70</td>
          <td>0x5101</td>
          <td>0x5100</td>
          <td>64</td>
          <td>Progamm HK0</td>
          <td>AUS</td>
        </tr>
        <tr>
          <td>71</td>
          <td>0x5121</td>
          <td>0x5120</td>
          <td>64</td>
          <td>Progamm HK1</td>
          <td>HEIZEN</td>
        </tr>
        <tr>
          <td>72</td>
          <td>0x5141</td>
          <td>0x5140</td>
          <td>64</td>
          <td>Progamm HK2</td>
          <td>HEIZEN</td>
        </tr>
        <tr>
          <td>73</td>
          <td>0x5161</td>
          <td>0x5160</td>
          <td>64</td>
          <td>Progamm HK3</td>
          <td>AUS</td>
        </tr>
        <tr>
          <td>74</td>
          <td>0x5181</td>
          <td>0x5180</td>
          <td>64</td>
          <td>Progamm HK4</td>
          <td>AUS</td>
        </tr>
        <tr>
          <td>75</td>
          <td>0x51a1</td>
          <td>0x51a0</td>
          <td>64</td>
          <td>Progamm HK5</td>
          <td>AUS</td>
        </tr>
        <tr>
          <td>76</td>
          <td>0x51c1</td>
          <td>0x51c0</td>
          <td>64</td>
          <td>Progamm HK6</td>
          <td>AUS</td>
        </tr>
        <tr>
          <td>77</td>
          <td>0x51e1</td>
          <td>0x51e0</td>
          <td>64</td>
          <td>Progamm HK7</td>
          <td>AUS</td>
        </tr>
        <tr>
          <td>78</td>
          <td>0x5201</td>
          <td>0x5200</td>
          <td>64</td>
          <td>Progamm HK8</td>
          <td>AUS</td>
        </tr>
      </tbody>
    </table>
  </body>
</html>
