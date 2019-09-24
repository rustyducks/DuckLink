from duckmsg import DuckMsg, clamp

class downToto(DuckMsg):
	def __init__(self):
		self.id = 0
		self._decimal = 0.0
		self._entier = 0
		self._name = b''
		self._vx = 0.0

	@property
	def decimal(self):
		return self._decimal

	@decimal.setter
	def decimal(self, decimal):
		self._decimal=clamp(-340282346638528860000000000000000000000, decimal, 340282346638528860000000000000000000000)

	@property
	def entier(self):
		return self._entier

	@entier.setter
	def entier(self, entier):
		self._entier=clamp(-32768, entier, 32767)

	@property
	def name(self):
		return self._name

	@name.setter
	def name(self, name):
		self._name=name

	@property
	def vx(self):
		return self._vx

	@vx.setter
	def vx(self, vx):
		self._vx=clamp(-340282346638528860000000000000000000000, vx, 340282346638528860000000000000000000000)

class interMCUProut(DuckMsg):
	def __init__(self):
		self.id = 1
		self._odeur = b''

	@property
	def odeur(self):
		return self._odeur

	@odeur.setter
	def odeur(self, odeur):
		self._odeur=odeur

class upPlop(DuckMsg):
	def __init__(self):
		self.id = 2
		self._decimal = 0.0
		self._entier = 0
		self._name = b''

	@property
	def decimal(self):
		return self._decimal

	@decimal.setter
	def decimal(self, decimal):
		self._decimal=clamp(-30, decimal, 1000)

	@property
	def entier(self):
		return self._entier

	@entier.setter
	def entier(self, entier):
		self._entier=clamp(-32768, entier, 32767)

	@property
	def name(self):
		return self._name

	@name.setter
	def name(self, name):
		self._name=name

class upSpeedReport(DuckMsg):
	def __init__(self):
		self.id = 3
		self._vtheta = 0
		self._vx = 0
		self._vy = 0

	@property
	def vtheta(self):
		return self._vtheta

	@vtheta.setter
	def vtheta(self, vtheta):
		self._vtheta=clamp(0, vtheta, 10)

	@property
	def vx(self):
		return self._vx

	@vx.setter
	def vx(self, vx):
		self._vx=clamp(-128, vx, 127)

	@property
	def vy(self):
		return self._vy

	@vy.setter
	def vy(self, vy):
		self._vy=clamp(-2, vy, 10)
